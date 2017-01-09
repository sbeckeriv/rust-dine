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


#[derive(Insertable, Debug, Clone)]
#[table_name="violations"]
pub struct NewViolation {
    pub inspection_id: i32,
    pub kind: String,
    pub points: i32,
    pub description: String,
}
impl NewViolation {
    pub fn insert(&self) -> Option<Violation> {
        diesel::insert(self).into(violations::table).get_result::<Violation>(&db()).ok()
    }
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

impl Violation {
    pub fn find_from_xml(inspection_id: i32, violation: &ViolationXml) -> Option<Violation> {
        all_violations.filter(
            violations::kind.eq(violation.violation_type.clone().unwrap_or("".to_string()))
                .and(violations::description.eq(violation.violation_descr.clone().unwrap_or("".to_string())))
                .and(violations::inspection_id.eq(inspection_id))
                .and(violations::points.eq(violation.violation_points.clone().unwrap_or("".to_string()) .parse::<i32>() .unwrap_or(0))))
            .first::<Violation>(&db())
            .ok()
    }

    pub fn find_or_create(inspection: &Inspection, violation: &ViolationXml) -> Violation {
        let record = Violation::find_from_xml(inspection.id, &violation);
        match record {
            Some(record) => record,
            None => {
                let mut new_record = NewViolation {
                    inspection_id: inspection.id,
                    kind: violation.violation_type.clone().unwrap_or("".to_string()),
                    description: violation.violation_descr.clone().unwrap_or("".to_string()),
                    points: violation.violation_points
                        .clone()
                        .unwrap_or("".to_string())
                        .parse::<i32>()
                        .unwrap_or(0),
                };
                new_record.insert().unwrap()
            }
        }
    }
}

#[derive(Insertable,  Debug, Clone)]
#[table_name = "inspections"]
pub struct NewInspection {
    pub place_id: i32,
    pub title: String,
    pub published: bool,
    pub closed: bool,
    pub inspected_at: NaiveDateTime,
    pub inspection_type: String,
    pub inspection_score: i32,
}

impl NewInspection {
    pub fn insert(&self) -> Option<Inspection> {
        diesel::insert(self).into(inspections::table).get_result::<Inspection>(&db()).ok()
    }
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
    pub fn find_from_xml(place_id: i32, inspection_date: &NaiveDateTime) -> Option<Inspection> {
        all_inspections.filter(inspections::inspected_at.eq(inspection_date)
                .clone()
                .and(inspections::place_id.eq(place_id)))
            .first::<Inspection>(&db())
            .ok()
    }

    pub fn find_or_create(place: &Place, inspection: &InspectionXml) -> Inspection {
        let date_string = inspection.inspection_date.clone().unwrap_or("".to_string());
        let mut date_string = date_string.trim().to_string();
        date_string.push_str(" 7:15");

        println!("{:?}", date_string);
        let inspection_date = NaiveDateTime::parse_from_str(&date_string, "%m/%d/%Y %H:%M")
            .unwrap_or(NaiveDate::from_ymd(1999, 9, 5).and_hms(23, 56, 4));
        println!("{:?}", inspection_date);
        let record = Inspection::find_from_xml(place.id, &inspection_date);
        match record {
            Some(record) => record,
            None => {
                let mut new_record = NewInspection {
                    place_id: place.id,
                    inspected_at: inspection_date,
                    title: "".to_string(),
                    published: true,
                    closed: false,
                    inspection_type: inspection.inspection_type.clone().unwrap_or("".to_string()),
                    // pub inspected_at: NaiveDateTime,
                    inspection_score: inspection.inspection_score
                        .clone()
                        .unwrap_or("".to_string())
                        .parse::<i32>()
                        .unwrap_or(0),
                };
                new_record.insert().unwrap()
            }
        }
    }
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
        all_places.filter(places::address.eq(business.address.clone().unwrap_or("".to_string()))
                .and(places::name.eq(business.name.clone().unwrap_or("".to_string()))))
            .first::<Place>(&db())
            .ok()
    }

    pub fn find_or_create(business: &BusinessXml) -> Place {
        let place = Place::find_from_xml(&business);
        match place {
            Some(place) => place,
            None => {
                let mut new_place = NewPlace {
                    name: business.name.clone().unwrap_or("".to_string()),
                    program_identifier: business.program_identifier
                        .clone()
                        .unwrap_or("".to_string()),
                    description: business.description.clone(),
                    phone: business.phone.clone(),
                    address: business.address.clone().unwrap_or("".to_string()),
                    longitude: business.longitude
                        .clone()
                        .unwrap_or("".to_string())
                        .parse::<f64>()
                        .unwrap_or(0.0),
                    latitude: business.latitude
                        .clone()
                        .unwrap_or("".to_string())
                        .parse::<f64>()
                        .unwrap_or(0.0),
                };
                new_place.insert().unwrap()
            }
        }
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
    pub inspection_date: Option<String>,
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
