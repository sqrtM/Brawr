use std::time::SystemTime;

use crate::schema::sessions;
use diesel::prelude::*;
use rocket::serde::Deserialize;

use serde::Serialize;

#[derive(Queryable, Insertable, Selectable, Identifiable, Deserialize, Serialize, Debug)]
#[diesel(primary_key(session_id))]
#[diesel(table_name = sessions)]
#[diesel(belongs_to(User))]
pub struct Session {
    pub session_id: i32,
    pub user_id: i32,
    pub expires_at: SystemTime,
    pub created_at: chrono::NaiveDateTime,
    pub secret: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: i32,
    pub expires_at: chrono::NaiveDateTime,
    pub secret: String,
}
