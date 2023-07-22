use rocket::serde::json::Json;

use crate::{models::user::UserRequest, services::user::add_user as add_user_service};

#[post("/user", data = "<request>")]
pub fn add_user(request: Json<UserRequest>) -> String {
    match add_user_service::execute(request) {
        Ok(e) => format!("{:#?}", e),
        Err(_) => "error!!!!".to_owned(),
    }
}
