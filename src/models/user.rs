use crate::schema::users;
use diesel::prelude::*;
use rocket::serde::Deserialize;


#[derive(Queryable, Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::NaiveDateTime>
}
