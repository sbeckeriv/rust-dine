use std::time::{Duration, SystemTime};
use std::env;
use std::collections::HashMap;
use std::process::Command;
use toml;
use glob::glob;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AssetType {
    extention: String,
    build_command: Vec<String>,
}

#[derive(Debug)]
pub struct AssetPipeline {
    pub registered_files: HashMap<String, Asset>,
}

unsafe impl Sync for AssetPipeline {}
impl AssetPipeline {
    pub fn new(mainifest_location: &str) -> Result<AssetPipeline, io::Error> {
        let mut f = try!(File::open(mainifest_location));
        let mut toml = String::new();
        try!(f.read_to_string(&mut toml));
        let mut pipeline = AssetPipeline { registered_files: HashMap::new() };
        let mainifest: toml::Value = toml.parse().unwrap();
        let files = mainifest.lookup("files").unwrap();

        for file in files.as_slice().unwrap().iter() {
            let table = file.as_table().unwrap();
            let name = table.get("path").unwrap().as_str().unwrap().to_string();
            let commands = table.get("commands")
                .unwrap()
                .as_slice()
                .unwrap()
                .to_vec()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
            let watch = table.get("watch")
                .unwrap()
                .as_slice()
                .unwrap()
                .to_vec()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
            pipeline.register_file(Asset::new(name, watch, commands));
        }
        // add types
        // add files
        Ok(pipeline)
    }

    pub fn register_file(&mut self, asset: Asset) -> Option<()> {
        self.registered_files.insert(asset.path.clone(), asset);
        None
    }
    pub fn file_for(&mut self, requested_path: &str) -> Option<String> {
        if let Some(mut asset) = self.registered_files.get_mut(requested_path) {
            println!("{:?}", asset.last_modified);
            if let Some(asset_last_modified) = asset.last_modified {
                if let Some(last_modified_file) = Self::last_modified_file(&asset.watchers) {
                    println!("{:?} {:?}", last_modified_file, asset_last_modified);
                    if last_modified_file > asset_last_modified {
                        Self::run_command(&asset.commands);
                        asset.last_modified = Some(last_modified_file);
                    }
                } else {
                    println!("no file mod time ");
                }

            } else {
                Self::run_command(&asset.commands);
                asset.last_modified = Some(SystemTime::now());
            }
        } else {
            println!("no file for {}", requested_path);
        }
        None
    }

    fn last_modified_file(watch: &Vec<String>) -> Option<SystemTime> {
        let mut last: Option<SystemTime> = None;
        for glob_string in watch {

            println!("{:?}", glob_string);
            for entry in glob(glob_string).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        println!("{:?}", path);
                        let metadata = path.metadata().expect("metadata call failed");
                        if metadata.is_file() {
                            if let Ok(time) = metadata.modified() {
                                match last {
                                    None => {
                                        last = Some(time);
                                    }
                                    Some(last_time) => {
                                        if time > last_time {
                                            last = Some(time);
                                        }
                                    }
                                }
                            } else {
                                println!("File modified time not supported on this platform");
                            }
                        }
                    }
                    Err(e) => {

                        println!("{}", e);
                    }
                }
            }
        }
        last
    }

    fn run_command(commands: &Vec<String>) {
        let mut child = Command::new(commands.get(1).unwrap())
            .current_dir(commands.get(0).unwrap())
            .spawn()
            .expect("failed to execute child");

        let ecode = child.wait()
            .expect("failed to wait on child");

        assert!(ecode.success());
    }
}

#[derive( Eq, PartialEq, Debug)]
pub struct Asset {
    path: String,
    watchers: Vec<String>,
    commands: Vec<String>,
    last_modified: Option<SystemTime>,
}
unsafe impl Sync for Asset {}
impl Asset {
    fn new(path: String, watch: Vec<String>, commands: Vec<String>) -> Asset {
        Asset {
            commands: commands,
            path: path,
            last_modified: None,
            watchers: watch,
        }
    }
}
