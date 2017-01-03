#![feature(plugin, custom_derive, proc_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate dotenv;
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
use std::collections::HashMap;
use std::sync::Mutex;
use tera::Context;

pub mod schema;
pub mod models;
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
struct PlaceDetailsJSON {
    pub inspections: Vec<models::Inspection>,
    pub id: i32,
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

#[get("/location?<lat_long>")]
fn location(lat_long: LatLongParams) -> JSON<PlacesJSON> {
    let places = models::Place::in_the_bounds(lat_long.sw_long,
                                              lat_long.ne_long,
                                              lat_long.ne_lat,
                                              lat_long.sw_lat,
                                              lat_long.min,
                                              lat_long.max);
    let mut place = models::NewPlace {
        name: "String".to_string(),
        program_identifier: "String".to_string(),
        description: None,
        phone: None,
        address: "String".to_string(),
        longitude: -122.3851447207237,
        latitude: 47.66657874084547,
    };
    place.insert();
    let json = places.iter()
        .map(|record| {
            let ref place = record.0;
            let ref inspections = record.1;
            PlaceDetailsJSON {
                inspections: inspections.clone(),
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
    JSON(data)
}

#[derive(FromForm)]
struct SearchParams<'r> {
    captures: &'r str,
    limit: Option<i64>,
}
#[get("/search?<search>")]
fn search(search: SearchParams) -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, location, search, static_files::all]).launch();
}
