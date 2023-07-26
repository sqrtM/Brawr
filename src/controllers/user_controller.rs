use rocket::{
    response::status::{Accepted, BadRequest},
    serde::json::Json,
};

use crate::{
    models::user::{User, UserRequest, UserResponse, UserResponseWithSecret},
    services::user::{
        add_user as add_user_service, get_user_by_id as get_user_by_id_service,
        get_users as get_users_service,
    },
};

#[post("/", data = "<request>")]
pub fn add_user(
    request: Json<UserRequest>
) -> Result<Accepted<Json<UserResponseWithSecret>>, BadRequest<Json<String>>> {
    if !UserRequest::is_valid(&request) {
        let json: Option<Json<String>> =
            Some(Json("Error: Email or Password are invalid".to_string()));
        return Err(BadRequest(json));
    }
    match add_user_service::execute(request) {
        Ok(users) => {
            let json: Option<Json<UserResponseWithSecret>> = Some(Json(users));
            Ok(Accepted(json))
        }
        Err(e) => {
            let json: Option<Json<String>> = Some(Json(e));
            Err(BadRequest(json))
        }
    }
}

#[get("/")]
pub fn get_users() -> Result<Accepted<Json<Vec<User>>>, BadRequest<Json<String>>> {
    match get_users_service::execute() {
        Ok(users) => {
            let json: Option<Json<Vec<User>>> = Some(Json(users));
            Ok(Accepted(json))
        }
        Err(e) => {
            let json: Option<Json<String>> = Some(Json(format!("Error: {}", e.to_string())));
            Err(BadRequest(json))
        }
    }
}

#[get("/<id>")]
pub fn get_user_by_id(id: i32) -> Result<Accepted<Json<UserResponse>>, BadRequest<Json<String>>> {
    match get_user_by_id_service::execute(id) {
        Ok(user) => {
            let json: Option<Json<UserResponse>> = Some(Json(user));
            Ok(Accepted(json))
        }
        Err(e) => {
            let json: Option<Json<String>> = Some(Json(format!("Error: {}", e.to_string())));
            Err(BadRequest(json))
        }
    }
}
