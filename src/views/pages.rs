use crate::config::Config;
use crate::views::context::NoContext;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[get("/wip")]
pub fn wip() -> Template {
    Template::render("wip", NoContext {})
}
