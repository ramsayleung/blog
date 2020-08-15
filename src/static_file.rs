use rocket::get;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<path..>", rank = 5)]
pub fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
