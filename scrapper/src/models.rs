use diesel;
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

// SHOULDDO: Break out the xml in this file. Move changes to main folder and figure out how to load them.

// could make this a join table and a single list of violations
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
                let new_record = NewViolation {
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

    pub fn find_or_create(place: &Place, inspection: &InspectionXml) -> (Inspection, bool) {
        let date_string = inspection.inspection_date.clone().unwrap_or("".to_string());
        let mut date_string = date_string.trim().to_string();
        date_string.push_str(" 7:15");
        let inspection_date = NaiveDateTime::parse_from_str(&date_string, "%m/%d/%Y %H:%M")
            .unwrap_or(NaiveDate::from_ymd(1999, 9, 5).and_hms(23, 56, 4));
        let record = Inspection::find_from_xml(place.id, &inspection_date);
        match record {
            Some(record) => (record, false),
            None => {
                let new_record = NewInspection {
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
                (new_record.insert().unwrap(), true)
            }
        }
    }
    pub fn all() -> Vec<Inspection> {
        all_inspections.order(inspections::id.desc()).load::<Inspection>(&db()).unwrap()
    }

    pub fn insert(&self) -> bool {
        diesel::insert(self).into(inspections::table).execute(&db()).is_ok()
    }
}

#[derive(Insertable, Debug, Clone)]
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
                let lat = business.latitude
                    .clone()
                    .unwrap_or("".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let long = business.longitude
                    .clone()
                    .unwrap_or("".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.0);
                let new_place = NewPlace {
                    name: business.name.clone().unwrap_or("".to_string()),
                    program_identifier: business.program_identifier
                        .clone()
                        .unwrap_or("".to_string()),
                    description: business.description.clone(),
                    phone: business.phone.clone(),
                    address: business.address.clone().unwrap_or("".to_string()),
                    longitude: long,
                    latitude: lat,
                };
                new_place.insert().unwrap()
            }
        }
    }
}


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ViolationXml {
    #[serde(rename = "Violation_Type")]
    pub violation_type: Option<String>,
    #[serde(rename = "Violation_Descr")]
    pub violation_descr: Option<String>,
    #[serde(rename = "Violation_Points")]
    pub violation_points: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct InspectionXml {
    #[serde(rename = "Inspection_Date")]
    pub inspection_date: Option<String>,
    #[serde(rename = "Inspection_Business_Name")]
    pub inspection_business_name: Option<String>,
    #[serde(rename = "Inspection_Type")]
    pub inspection_type: Option<String>,
    #[serde(rename = "Inspection_Score")]
    pub inspection_score: Option<String>,
    #[serde(rename = "Inspection_Result")]
    pub inspection_result: Option<String>,
    #[serde(rename = "Inspection_Closed_Business")]
    pub inspection_closed_business: Option<String>,
    #[serde(rename = "Violation")]
    pub violation: Option<Vec<ViolationXml>>,
}

#[derive(Debug,  Deserialize, PartialEq, Serialize)]
pub struct BusinessXml {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Program_Identifier")]
    pub program_identifier: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "City")]
    pub city: Option<String>,
    #[serde(rename = "Zip_Code")]
    pub zip_code: Option<String>,
    #[serde(rename = "Phone")]
    pub phone: Option<String>,
    // fails if there is no long or lat. need to parse them as strings then convert to floats
    #[serde(rename = "Longitude")]
    pub longitude: Option<String>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<String>,
    #[serde(rename = "Inspection")]
    pub inspection: Option<Vec<InspectionXml>>,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BusinessInspectionViolation {
    #[serde(rename = "Disclaimer")]
    pub disclaimer: Option<String>,
    #[serde(rename = "Business")]
    pub business: Vec<BusinessXml>,
}
