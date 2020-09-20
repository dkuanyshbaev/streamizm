use crate::config::Config;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct ItemContext<T> {
    pub item: T,
}

#[derive(Serialize)]
struct ListContext<T> {
    pub items: Vec<T>,
}

#[derive(Serialize)]
struct NoContext {}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[get("/home")]
pub fn home() -> Template {
    Template::render("home", NoContext {})
}

#[get("/play")]
pub fn play() -> Template {
    Template::render("play", NoContext {})
}

#[get("/events")]
pub fn events() -> Template {
    Template::render("events", NoContext {})
}

#[get("/collabs")]
pub fn collabs() -> Template {
    Template::render("collabs", NoContext {})
}

#[get("/terms")]
pub fn terms() -> Template {
    Template::render("terms", NoContext {})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render("500", NoContext {})
}
