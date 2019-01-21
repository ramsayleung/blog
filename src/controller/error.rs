use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket::Request;
#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("admin/404", &map)
}
#[catch(401)]
pub fn unauthorised(_req: &Request) -> Redirect {
    Redirect::to("/admin/login")
}
