mod controllers;
mod models;
mod schema;
mod services;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Build, Rocket};
use rocket::{Request, Response};

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .mount(
            "/api/user",
            routes![
                controllers::user_controller::add_user,
                controllers::user_controller::get_users,
                controllers::user_controller::get_user_by_id,
            ],
        )
        .mount(
            "/api/character",
            routes![
                controllers::character_controller::add_character,
                controllers::character_controller::get_characters,
                controllers::character_controller::get_character_by_id
            ],
        )
}
