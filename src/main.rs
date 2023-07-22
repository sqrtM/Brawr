mod models;
mod schema;
mod services;

use diesel::RunQueryDsl;
use models::user::User;
use rocket::{serde::json::Json, Build, Rocket};

use crate::services::pg_connection::establish_connection_pg;

#[macro_use]
extern crate rocket;

#[post("/add-user", format = "json", data = "<user>")]
fn add_user(user: Json<User>) -> String {
    let new_user = User {
        user_id: user.user_id,
        email: user.email.to_string(),
        password: user.password.to_string(),
        created_at: match user.created_at {
            Some(dt) => Some(dt),
            None => chrono::NaiveDateTime::from_timestamp_millis(0)
        }
    };
    let conn: &mut diesel::PgConnection = &mut establish_connection_pg();
    diesel::insert_into(schema::users::table).values(&new_user).execute(conn).expect("err");
    format!("user added {:?}", new_user)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/api", routes![add_user])
}
