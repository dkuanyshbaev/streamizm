#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::{env, process};

#[derive(Serialize)]
struct NoContext {}

#[derive(Debug, FromForm)]
pub struct Letter {
    name: String,
    email: String,
    message: String,
}

pub struct Config {
    pub email: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Result<Config, std::env::VarError> {
        let email = env::var("EMAIL")?;
        let password = env::var("PASSWORD")?;
        Ok(Config { email, password })
    }
}

#[launch]
fn rocket() -> rocket::Rocket {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Can't parsing config: {}", err);
        process::exit(1);
    });

    rocket::ignite()
        .manage(config)
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![index, live, about, contact, send])
        .mount("/ru", routes![ru_index, ru_live, ru_about, ru_contact])
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

#[get("/about/<id>")]
pub fn about(id: u8) -> Template {
    match id {
        1 => Template::render("about1", NoContext {}),
        2 => Template::render("about2", NoContext {}),
        3 => Template::render("about3", NoContext {}),
        _ => Template::render("about4", NoContext {}),
    }
}

#[get("/contact")]
pub fn contact() -> Template {
    Template::render("contact", NoContext {})
}

#[get("/")]
pub fn ru_index() -> Template {
    Template::render("ru_index", NoContext {})
}

#[get("/live")]
pub fn ru_live() -> Template {
    Template::render("ru_live", NoContext {})
}

#[get("/about/<id>")]
pub fn ru_about(id: u8) -> Template {
    match id {
        1 => Template::render("ru_about1", NoContext {}),
        2 => Template::render("ru_about2", NoContext {}),
        3 => Template::render("ru_about3", NoContext {}),
        _ => Template::render("ru_about4", NoContext {}),
    }
}

#[get("/contact")]
pub fn ru_contact() -> Template {
    Template::render("ru_contact", NoContext {})
}

#[post("/send", data = "<letter>")]
pub fn send(config: State<Config>, letter: Form<Letter>) -> Redirect {
    let email = Message::builder()
        .from(config.email.parse().unwrap())
        .to(config.email.parse().unwrap())
        .subject("101Seasons.org - Contact message")
        .body(format!(
            "Name: {}. Email: {}. Message: {}",
            letter.name, letter.email, letter.message
        ))
        .unwrap();

    let creds = Credentials::new(config.email.to_string(), config.password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }

    Redirect::to("/")
}

#[catch(404)]
pub fn not_found() -> Template {
    Template::render("404", NoContext {})
}
