#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[cfg(test)]
mod tests;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;

mod static_file;
#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>,
}
#[derive(Serialize)]
struct Context {}
#[get("/")]
fn index() -> Redirect {
    Redirect::to("/index")
}

#[get("/admin/index")]
fn admin() -> Template {
    let name = String::from("Helloworld");
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    Template::render("admin/index", &context)
}
#[get("/index")]
fn get() -> Template {
    let context = Context {};
    Template::render("index", &context)
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("admin/404", &map)
}

// #[get("/static/<file..>")]
// fn static_content(file: PathBuf) -> Result<NamedFile> {
//     NamedFile::open(Path::new("static/").join(file)).chain_err(|| "File not found!")
// }

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get, static_file::all, admin])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
