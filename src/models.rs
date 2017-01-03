use diesel;
use diesel::types::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use schema::inspections;
use schema::inspections::dsl::inspections as all_inspections;
use schema::places;
use schema::places::dsl::places as all_places;

fn db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Serialize, Identifiable, Associations, Deserialize, Queryable, Insertable, FromForm, Debug, Clone)]
#[belongs_to(Place)]
#[table_name = "inspections"]
pub struct Inspection {
    pub id: i32,
    pub place_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
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
    pub fn insert(&self) -> bool {
        diesel::insert(self).into(places::table).execute(&db()).is_ok()
    }
}

#[derive(Serialize,Associations, Identifiable, Deserialize, Queryable, FromForm, Debug, Clone)]
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
