use diesel;
use diesel::types::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use std::env;
use schema::inspections;
use schema::inspections::dsl::inspections as all_inspections;
use schema::violations;
use schema::violations::dsl::violations as all_violations;
use schema::places;
use schema::places::dsl::places as all_places;

fn db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize, Identifiable, Associations, Deserialize, Queryable, Insertable, Debug, Clone)]
#[belongs_to(Inspection)]
#[table_name="violations"]
pub struct Violation {
    pub id: i32,
    pub inspection_id: i32,
    pub kind: String,
    pub points: i32,
    pub description: String,
}

#[derive(Serialize, Identifiable, Associations, Deserialize, Queryable, Insertable,  Debug, Clone)]
#[belongs_to(Place)]
#[table_name = "inspections"]
pub struct Inspection {
    pub id: i32,
    pub place_id: i32,
    pub title: String,
    pub published: bool,
    pub closed: bool,
    pub inspected_at: NaiveDateTime,
    pub inspection_type: String,
    pub inspection_score: i32,
}

impl Inspection {
    pub fn all() -> Vec<Inspection> {
        all_inspections.order(inspections::id.desc()).load::<Inspection>(&db()).unwrap()
    }

    pub fn insert(&self) -> bool {
        diesel::insert(self).into(inspections::table).execute(&db()).is_ok()
    }

    pub fn delete_with_id(id: i32) -> bool {
        diesel::delete(all_inspections.find(id)).execute(&db()).is_ok()
    }
}

#[derive(Insertable, FromForm, Debug, Clone)]
#[table_name = "places"]
pub struct NewPlace {
    pub name: String,
    pub program_identifier: String,
    pub description: Option<String>,
    pub phone: Option<String>,
    pub address: String,
    pub longitude: f64,
    pub latitude: f64,
}
impl NewPlace {
    pub fn insert(&self) -> Option<Place> {
        diesel::insert(self).into(places::table).get_result::<Place>(&db()).ok()
    }
}

#[derive(Serialize,Associations, Identifiable, Deserialize, Queryable, Debug, Clone)]
#[table_name = "places"]
#[has_many(inspections)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub program_identifier: String,
    pub description: Option<String>,
    pub phone: Option<String>,
    pub address: String,
    pub longitude: f64,
    pub latitude: f64,
}
impl Place {
    pub fn find_from_xml(business: &BusinessXml) -> Option<Place> {

        all_places.filter(places::address.eq(business.address.unwrap_or("".to_string()))
                .and(places::name.eq(business.name.unwrap_or("".to_string())))
                .and(places::name.eq(business.program_identifier.unwrap_or("".to_string()))))
            .first::<Place>(&db())
            .ok()
    }

    pub fn find_or_create(business: &BusinessXml) -> Place {
        let place = Place::find_from_xml(&business);
        let mut place = NewPlace {
            name: business.name.unwrap_or("".to_string()),
            program_identifier: business.program_identifier.unwrap_or("".to_string()),
            description: business.description,
            phone: business.phone,
            address: business.address.unwrap_or("".to_string()),
            longitude: business.longitude.unwrap_or("".to_string()).parse::<f64>().unwrap_or(0.0),
            latitude: business.latitude.unwrap_or("".to_string()).parse::<f64>().unwrap_or(0.0),
        };
        place.insert();
    }
    pub fn in_the_bounds(sw_long: f64,
                         ne_long: f64,
                         ne_lat: f64,
                         sw_lat: f64,
                         min: Option<i64>,
                         max: Option<i64>)
                         -> Vec<(Place, Vec<Inspection>)> {
        let places = all_places.filter(places::longitude.ge(sw_long)
                .and(places::longitude.le(ne_long))
                .and(places::latitude.le(ne_lat))
                .and(places::latitude.ge(sw_lat)))
            .order(places::id.desc())
            .load::<Place>(&db())
            .unwrap();
        let inspection_list = Inspection::belonging_to(&places).load::<Inspection>(&db()).unwrap();
        let grouped = inspection_list.grouped_by(&places);
        places.into_iter().zip(grouped).collect::<Vec<_>>()
    }
}


#[derive(Debug,Display, Deserialize, PartialEq, Serialize)]
pub struct ViolationXml {
    pub violation_type: Option<String>,
    pub violation_descr: Option<String>,
    pub violation_points: Option<String>,
}

#[derive(Debug,Display, Deserialize, PartialEq, Serialize)]
pub struct InspectionXml {
    pub Inspection_date: Option<String>,
    pub inspection_business_name: Option<String>,
    pub inspection_type: Option<String>,
    pub inspection_score: Option<String>,
    pub inspection_result: Option<String>,
    pub inspection_closed_business: Option<String>,
    pub violation: Option<Vec<ViolationXml>>,
}

#[derive(Debug, Display, Deserialize, PartialEq, Serialize)]
pub struct BusinessXml {
    pub name: Option<String>,
    pub program_identifier: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub phone: Option<String>,
    pub longitude: Option<String>,
    pub latitude: Option<String>,
    pub inspection: Option<Vec<InspectionXml>>,
}
#[derive(Debug,Display, Deserialize, PartialEq, Serialize)]
pub struct BusinessInspectionViolation {
    pub disclaimer: Option<String>,
    pub business: Vec<BusinessXml>,
}
