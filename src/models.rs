use diesel;
use diesel::types::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use std::time::Duration;
use std::env;
use schema::inspections;
use schema::inspections::dsl::inspections as all_inspections;
use schema::violations;
use schema::violations::dsl::violations as all_violations;
use schema::places;
use schema::places::dsl::places as all_places;
use r2d2::{self, PooledConnection};
use r2d2_diesel::ConnectionManager;

lazy_static! {
  static ref DB_POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
    let database_url = env::var("DATABASE_URL").expect("Find DATABASE_URL environment variable");

    let config = r2d2::Config::builder()
      .pool_size(10)
      .connection_timeout(Duration::from_secs(5))
      .build();

    let manager = ConnectionManager::<PgConnection>::new(
      database_url
    );

    r2d2::Pool::new(config, manager).expect("Create database pool")
  };
}

pub struct DBConnection(PooledConnection<ConnectionManager<PgConnection>>);

impl DBConnection {
    pub fn get(&self) -> &PooledConnection<ConnectionManager<PgConnection>> {
        &self.0
    }
}

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
impl Violation {
    pub fn for_inspections(inspections: &Vec<Inspection>) -> Vec<(&Inspection, Vec<Violation>)> {
        let ref local_db = *DB_POOL.get().unwrap();
        let violations_list = Violation::belonging_to(inspections)
            .load::<Violation>(local_db)
            .unwrap();
        let grouped = violations_list.grouped_by(inspections);
        inspections.into_iter().zip(grouped).collect::<Vec<_>>()
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
    pub fn all() -> Vec<Inspection> {
        let ref local_db = *DB_POOL.get().unwrap();
        all_inspections.order(inspections::id.desc())
            .load::<Inspection>(local_db)
            .unwrap()
    }

    pub fn insert(&self) -> bool {

        let ref local_db = *DB_POOL.get().unwrap();
        diesel::insert(self).into(inspections::table).execute(local_db).is_ok()
    }

    pub fn delete_with_id(id: i32) -> bool {

        let ref local_db = *DB_POOL.get().unwrap();
        diesel::delete(all_inspections.find(id)).execute(local_db).is_ok()
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
        let ref local_db = *DB_POOL.get().unwrap();
        diesel::insert(self).into(places::table).execute(local_db).is_ok()
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
    pub fn in_the_bounds(sw_long: f64,
                         ne_long: f64,
                         ne_lat: f64,
                         sw_lat: f64,
                         min: Option<i64>,
                         max: Option<i64>)
                         -> Vec<(Place, Vec<(Inspection, Vec<Violation>)>)> {
        let ref local_db = *DB_POOL.get().unwrap();
        let places = all_places.filter(places::longitude.ge(sw_long)
                .and(places::longitude.le(ne_long))
                .and(places::latitude.le(ne_lat))
                .and(places::latitude.ge(sw_lat)))
            .order(places::id.desc())
            .load::<Place>(local_db)
            .unwrap();
        let inspection_list = Inspection::belonging_to(&places).load(local_db).unwrap();
        let violiations = Violation::belonging_to(&inspection_list).load(local_db).unwrap();
        let violiations: Vec<Vec<Violation>> = violiations.grouped_by(&inspection_list);
        let inspections_and_violiations: Vec<Vec<(Inspection, Vec<Violation>)>> =
            inspection_list.into_iter().zip(violiations).grouped_by(&places);
        let results: Vec<(Place, Vec<(Inspection, Vec<Violation>)>)> =
            places.into_iter().zip(inspections_and_violiations).collect();
        results
    }
}
