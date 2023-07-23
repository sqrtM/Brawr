mod controllers;
mod models;
mod schema;
mod services;

use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api/user",
            routes![
                controllers::user_controller::add_user,
                controllers::user_controller::get_users,
            ],
        )
        .mount(
            "/api/character",
            routes![controllers::character_controller::add_character],
        )
}
