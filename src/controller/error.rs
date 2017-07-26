use std::collections::HashMap;
use rocket_contrib::Template;
use rocket::Request;
#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("admin/404", &map)
}
