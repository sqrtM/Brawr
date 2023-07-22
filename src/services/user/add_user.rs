use std::time::SystemTime;

use diesel::{RunQueryDsl, result::Error};
use rocket::serde::json::Json;

use crate::{models::user::{User, UserRequest}, schema::{self}, services::pg_connection::establish_connection_pg};

pub fn execute(user: Json<UserRequest>) -> Result<Vec<User>, Error> {
    let new_user = User {
        user_id: user.user_id,
        email: user.email.to_string(),
        password: user.password.to_string(),
        created_at: SystemTime::now(),
    };
    let conn: &mut diesel::PgConnection = &mut establish_connection_pg();
    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_results(conn)
}
