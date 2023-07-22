mod models;
mod schema;
mod services;
mod controllers;

use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/api", 
        routes![
        controllers::user_controller::add_user,
        controllers::user_controller::get_users,
        ]
    )
}
