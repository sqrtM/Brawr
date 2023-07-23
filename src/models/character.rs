use crate::models::user::User;
use std::time::SystemTime;

use crate::schema::characters;
use diesel::prelude::*;
use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Insertable, Selectable, Identifiable, Associations, Deserialize, Serialize, Debug)]
#[diesel(primary_key(character_id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = characters)]
pub struct Character {
    pub character_id: i32,
    pub user_id: i32,
    pub character_name: String,
    pub constitution: i32,
    pub strength: i32,
    pub madness: i32,
    pub intelligence: i32,
    pub created_at: SystemTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = characters)]
pub struct NewCharacterRequest {
    pub user_id: i32,
    pub character_name: String,
    pub constitution: i32,
    pub strength: i32,
    pub madness: i32,
    pub intelligence: i32,
}
