#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod config;
mod errors;
mod views;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use views::{catchers, pages};

#[launch]
fn rocket() -> rocket::Rocket {
    let config = config::Config::new().unwrap_or_else(|err| {
        println!("Can't parsing config: {}", err);
        std::process::exit(1);
    });

    rocket::ignite()
        .manage(config)
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![pages::index,])
        .register(catchers![catchers::not_found, catchers::internal_error])
}
