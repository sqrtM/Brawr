use rocket::serde::json::Json;

use crate::{
    models::user::UserRequest,
    services::user::{
        add_user as add_user_service, 
        get_users as get_users_service,
        get_user_by_id as get_user_by_id_service
    },
};

#[post("/", data = "<request>")]
pub fn add_user(request: Json<UserRequest>) -> String {
    if !UserRequest::is_valid(&request) {
        return "not valid!!".to_owned();
    }
    match add_user_service::execute(request) {
        Ok(e) => format!("{:#?}", e),
        Err(e) => e.to_string(),
    }
}

#[get("/")]
pub fn get_users() -> String {
    match get_users_service::execute() {
        Ok(users) => format!("{:#?}", users),
        Err(e) => e.to_string(),
    }
}

#[get("/<id>")]
pub fn get_user_by_id(id: i32) -> String {
    match get_user_by_id_service::execute(id) {
        Ok(user) => format!("{:#?}", user),
        Err(e) => e.to_string(),
    }
}
