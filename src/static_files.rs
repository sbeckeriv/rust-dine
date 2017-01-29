use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};
use assets::AssetPipeline;
use rocket::config;
use std::sync::Mutex;
lazy_static! {
  static ref ASSET_PIPELINE: Mutex<AssetPipeline> = {
      Mutex::new(AssetPipeline::new("assets.toml").unwrap())
  };
}

lazy_static! {
  static ref USE_ASSET_PIPELINE: Option<bool> = {
    config::active().map(|config| {
        config.get_bool("live_assets")
            .unwrap_or(false)
    })
  };
}

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> io::Result<NamedFile> {
    if USE_ASSET_PIPELINE.unwrap_or(false) {
        ASSET_PIPELINE.lock().unwrap().file_for(path.to_str().unwrap());
    }
    NamedFile::open(Path::new("public/").join(path))
}
