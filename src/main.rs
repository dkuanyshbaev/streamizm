#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::{env, process};

#[derive(Serialize)]
struct NoContext {}

#[launch]
fn rocket() -> rocket::Rocket {
    let secret = env::var("SECRET").unwrap_or_else(|_| {
        println!("Secret is not set!");
        process::exit(1);
    });

    rocket::ignite()
        .manage(secret)
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![index, play, events])
        .register(catchers![not_found])
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[get("/play")]
pub fn play() -> Template {
    Template::render("play", NoContext {})
}

#[get("/events")]
pub fn events() -> Template {
    Template::render("events", NoContext {})
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}
