use rocket::fs::NamedFile;
use rocket::get;
use std::path::{Path, PathBuf};

#[get("/<path..>", rank = 5)]
pub async fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).await.ok()
}
