#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate chrono;
extern crate dotenv;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_diesel;
extern crate tera;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::{Template, JSON};
use rocket::http::Method;
use chrono::NaiveDateTime;

mod schema;
mod models;
mod cors;
mod static_files;

#[derive(Serialize)]
struct TemplateContext {
    items: Vec<String>,
}
#[get("/")]
fn index() -> Template {
    let context = TemplateContext {
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect(),
    };
    Template::render("index", &context)
}

#[derive(FromForm)]
struct LatLongParams {
    sw_long: f64,
    ne_long: f64,
    ne_lat: f64,
    sw_lat: f64,
    min: Option<i64>,
    max: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InspectionJSON {
    pub violations: Vec<models::Violation>,
    pub id: i32,
    pub place_id: i32,
    pub closed: bool,
    pub inspected_at: NaiveDateTime,
    pub inspection_type: String,
    pub inspection_score: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaceDetailsJSON {
    pub inspections: Vec<InspectionJSON>,
    pub id: i32,
    pub most_recent_score: i32,
    pub name: String,
    pub program_identifier: String,
    pub description: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlacesJSON {
    results: Vec<PlaceDetailsJSON>,
    status: i32,
    reason: Option<String>,
}

#[route(OPTIONS, "/inspections")]
fn inspections_options() -> cors::PreflightCORS {
    cors::CORS::preflight("http://localhost")
        .methods(vec![Method::Options, Method::Get])
        .headers(vec!["Content-Type"])
}

#[derive(FromForm)]
struct InspectionParams {
    place_id: i32,
}

#[get("/inspections?<inspection_params>")]
fn inspections(inspection_params: InspectionParams) -> cors::CORS<JSON<PlacesJSON>> {
    let places = models::Place::find_and_load(inspection_params.place_id);
    let json = places.iter()
        .map(|record| {
            let ref place = record.0;
            let ref inspections_and_violations = record.1;
            let inspections_json = inspections_and_violations.iter()
                .map(|i_record| {
                    let ref i = i_record.0;
                    let ref violations = i_record.1;
                    InspectionJSON {
                        violations: violations.clone(),
                        id: i.id,
                        place_id: i.place_id,
                        closed: i.closed,
                        inspected_at: i.inspected_at,
                        inspection_type: i.inspection_type.clone(),
                        inspection_score: i.inspection_score,
                    }
                })
                .collect();
            PlaceDetailsJSON {
                inspections: inspections_json,
                most_recent_score: -1,
                id: place.id,
                name: place.name.clone(),
                program_identifier: place.program_identifier.clone(),
                description: place.description.clone(),
                longitude: place.longitude,
                latitude: place.latitude,
            }
        })
        .collect();
    let data = PlacesJSON {
        results: json,
        status: 300,
        reason: None,
    };
    cors::CORS::any(JSON(data))

}

#[route(OPTIONS, "/location")]
fn cors_preflight() -> cors::PreflightCORS {
    cors::CORS::preflight("http://localhost")
        .methods(vec![Method::Options, Method::Get])
        .headers(vec!["Content-Type"])
}

#[get("/location?<lat_long>")]
fn location(lat_long: LatLongParams) -> cors::CORS<JSON<PlacesJSON>> {
    let places = models::Place::in_the_bounds(lat_long.sw_long,
                                              lat_long.ne_long,
                                              lat_long.ne_lat,
                                              lat_long.sw_lat,
                                              lat_long.min,
                                              lat_long.max);
    let json = places.iter()
        .map(|record| {
            let ref place = record.0;
            let ref inspections: Vec<models::Inspection> = record.1;
            let mut last_score = -1;
            // is this better then a db hit per object? should have db cache it
            let mut filtered_inspection: Vec<(NaiveDateTime, i32)> = inspections.iter()
                .filter_map(|&ref x: &models::Inspection| {
                    if !x.is_educational() {
                        Some((x.inspected_at.clone(), x.inspection_score))
                    } else {
                        None
                    }
                })
                .collect();
            filtered_inspection.sort_by(|a, b| b.0.cmp(&a.0));
            if let Some(x) = filtered_inspection.first() {
                last_score = x.1;
            }

            PlaceDetailsJSON {
                inspections: vec![],
                most_recent_score: last_score,
                id: place.id,
                name: place.name.clone(),
                program_identifier: place.program_identifier.clone(),
                description: place.description.clone(),
                longitude: place.longitude,
                latitude: place.latitude,
            }
        })
        .collect();
    let data = PlacesJSON {
        results: json,
        status: 300,
        reason: None,
    };
    cors::CORS::any(JSON(data))
}


fn main() {
    rocket::ignite()
        .mount("/",
               routes![index,
                       location,
                       cors_preflight,
                       inspections,
                       inspections_options,
                       static_files::all])
        .launch();
}
