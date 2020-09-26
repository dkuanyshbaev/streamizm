#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct NoContext {}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![index, live])
        .mount("/ru", routes![ru_index, ru_live])
        .register(catchers![not_found])
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[get("/live")]
pub fn live() -> Template {
    Template::render("live", NoContext {})
}

#[get("/")]
pub fn ru_index() -> Template {
    Template::render("ru_index", NoContext {})
}

#[get("/live")]
pub fn ru_live() -> Template {
    Template::render("ru_live", NoContext {})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}
