use rocket::serde::json::Json;

use crate::{models::user::UserRequest, services::user::{add_user as add_user_service, get_users as get_users_service}};

#[post("/user", data = "<request>")]
pub fn add_user(request: Json<UserRequest>) -> String {
    if !UserRequest::is_valid(&request) {
        return "not valid!!".to_owned();
    }
    match add_user_service::execute(request) {
        Ok(e) => format!("{:#?}", e),
        Err(_) => "error!!!!".to_owned(),
    }
}
#[get("/user")]
pub fn get_users() -> String {
    match get_users_service::execute() {
        Ok(e) => format!("{:#?}", e),
        Err(_) => "error!!!!".to_owned(),
    }
}
