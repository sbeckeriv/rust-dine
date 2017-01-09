#![feature(plugin, custom_derive, proc_macro)]
extern crate chrono;
extern crate dotenv;
extern crate serde;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml;
use std::fmt;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{NaiveDate, NaiveDateTime};
pub mod schema;
pub mod models;
pub mod error;

fn from_str(path: &str) -> Result<models::BusinessInspectionViolation, error::Error> {
    let file = Path::new(path);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    serde_xml::from_str::<models::BusinessInspectionViolation>(&buffer)
        .map_err(error::Error::XmlError)
}

fn main() {
    let x = from_str("single.xml");
    match x {
        Err(e) => println!("{}", e),
        Ok(xml) => {
            for business_xml in xml.business {
                let business = models::Place::find_or_create(&business_xml);
                println!("{:?}", business);
                if business_xml.inspection.is_some() {
                    for inspection_xml in business_xml.inspection.unwrap() {
                        let inspection = models::Inspection::find_or_create(&business,
                                                                            &inspection_xml);
                        println!("{:?}", inspection);
                        if inspection_xml.violation.is_some() {
                            for violation_xml in inspection_xml.violation.unwrap() {
                                models::Violation::find_or_create(&inspection, &violation_xml);
                            }
                        }
                    }
                }
            }
        }
    }
}
