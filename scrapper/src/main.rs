#![feature(plugin, custom_derive)]
extern crate dotenv;
extern crate chrono;
extern crate serde;
extern crate curl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml;
use chrono::*;

use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use curl::easy::Easy;

pub mod schema;
pub mod models;
pub mod error;

fn from_str(path: Option<String>) -> Result<models::BusinessInspectionViolation, error::Error> {
    let buffer = match path {
        Some(path) => {
            println!("loading file {}", path);
            let mut buffer = String::new();
            let file = Path::new(&path);
            let mut open_file = File::open(file).unwrap();
            open_file.read_to_string(&mut buffer).unwrap();
            println!("done loading file {}", path);
            buffer
        }
        None => {
            let now = chrono::Local::now();
            let url = format!("http://info.kingcounty.\
                               gov/health/ehs/foodsafety/inspections/Results.\
                               aspx?Output=X&Business_Name=%&Business_Address=&Longitude=&Latitude=&City=&Zip_Code=&Inspection_Type=All&Inspection_Start=1/1/2014&Inspection_End={}/{}/{}&Inspection_Closed_Business=A&Violation_Points=&Violation_Red_Points=&Violation_Descr=&Fuzzy_Search=N&Sort=B",
                              now.month(),
                              now.day(),
                              now.year());

            println!("starting download: {}", url);
            let mut dst = Vec::new();
            {
                let mut handle = Easy::new();
                handle.url(&url).unwrap();
                use std::str;
                let mut transfer = handle.transfer();
                transfer.write_function(|data| {
                        dst.extend_from_slice(data);
                        Ok(data.len())
                    })
                    .unwrap();
                transfer.perform().unwrap();
                println!("download complete");
            }
            String::from_utf8(dst).unwrap()
        }
    };
    serde_xml::from_str::<models::BusinessInspectionViolation>(&buffer)
        .map_err(error::Error::XmlError)
}

//
fn main() {
    let x = from_str(env::args().nth(1));
    println!("Done loading xml");
    match x {
        Err(e) => println!("{}", e),
        Ok(xml) => {
            // for business_xml in xml.business {
            xml.business
                .into_iter()
                //can i just run an iter with items?
                .map(|business_xml| {
                    //businesses can have the same lat long. we need to detect this an add an
                    //offset
                    let business = models::Place::find_or_create(&business_xml);
                    if business_xml.inspection.is_some() {
                        for inspection_xml in business_xml.inspection.unwrap() {
                            let (inspection,new_record) = models::Inspection::find_or_create(&business,
                                                                                &inspection_xml);

                            if new_record && inspection_xml.violation.is_some() {
                                for violation_xml in inspection_xml.violation.unwrap() {
                                    let points = violation_xml.violation_points.clone().unwrap_or("".to_string());
                                    if points != "" {
                                        models::Violation::find_or_create(&inspection, &violation_xml);
                                    }
                                }
                            }
                        }

                    }
                    let last_inspection_score = models::Inspection::find_most_recent_real_inspection_score(business.id);
                    business.update_last_score(last_inspection_score);
                    2
                })
                .collect::<Vec<i32>>();
        }
    }
}
