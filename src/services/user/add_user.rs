use std::time::SystemTime;

use chrono::{NaiveDate, NaiveTime};
use diesel::{result::Error, RunQueryDsl};
use rocket::serde::json::Json;

use crate::{
    models::{
        session::{NewSession, Session},
        user::{User, UserRequest}, character::Character,
    },
    schema::{sessions, users as user_schema},
    services::pg_connection::establish_connection_pg,
};

pub fn execute(user: Json<UserRequest>) -> Result<String, String> {
    let new_user = UserRequest {
        email: user.email.to_string(),
        password: user.password.to_string(),
    };
    let new_session = NewSession {
        user_id: 1,
        expires_at: NaiveDate::from_ymd_opt(2023, 7, 27)
            .unwrap()
            .and_hms_opt(0, 0, 3)
            .unwrap(),
        secret: sha256::digest(new_user.password.clone()),
    };
    let conn: &mut diesel::PgConnection = &mut establish_connection_pg();
    match diesel::insert_into(user_schema::table)
        .values(&new_user)
        .get_results::<User>(conn)
    {
        Ok(_) => {
            match diesel::insert_into(sessions::table)
                .values(&new_session)
                .execute(conn) {
                    Ok(_) => Ok("Yes".to_string()),
                    Err(_) => Err("No".to_string()),
                }
        }
        Err(_) => Err("No".to_string()),
    }
}
