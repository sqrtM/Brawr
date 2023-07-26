use regex::Regex;
use std::time::SystemTime;

use crate::schema::users;
use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize};

use serde::Serialize;

use super::character::Character;

#[derive(Queryable, Insertable, Selectable, Identifiable, Deserialize, Serialize, Debug)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

impl UserRequest {
    pub fn is_valid(request: &Json<UserRequest>) -> bool {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        let password_regex = Regex::new(r"^.*(.{8,})(.*[a-zA-Z])(.*\d)(.*[!#$%&? ]).*$").unwrap();
        email_regex.is_match(&request.email) && password_regex.is_match(&request.password)
    }
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct UserResponse {
    #[serde(flatten)]
    pub user: User,
    pub characters: Vec<Character>,
}


#[derive(Serialize)]
#[derive(Debug)]
pub struct UserResponseWithSecret {
    pub user: Vec<User>,
    pub secret: String
}