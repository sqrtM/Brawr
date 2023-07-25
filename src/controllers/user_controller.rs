use rocket::{
    response::status::{Accepted, BadRequest},
    serde::json::Json,
};

use crate::{
    models::user::UserRequest,
    services::user::{
        add_user as add_user_service, get_user_by_id as get_user_by_id_service,
        get_users as get_users_service,
    },
};

#[post("/", data = "<request>")]
pub fn add_user(
    request: Json<UserRequest>,
) -> Result<Accepted<Json<String>>, BadRequest<Json<String>>> {
    if !UserRequest::is_valid(&request) {
        return Err(BadRequest(Some(Json(
            "Error: Email or Password not Valid".to_string(),
        ))));
    }
    match add_user_service::execute(request) {
        Ok(e) => Ok(Accepted(Some(Json(format!("{:#?}", e))))),
        Err(e) => Err(BadRequest(Some(Json(format!("Error: {}", e.to_string()))))),
    }
}

#[get("/")]
pub fn get_users() -> Result<Accepted<Json<String>>, BadRequest<Json<String>>> {
    match get_users_service::execute() {
        Ok(users) => Ok(Accepted(Some(Json(format!("{:#?}", users))))),
        Err(e) => Err(BadRequest(Some(Json(format!("Error: {}", e.to_string()))))),
    }
}

#[get("/<id>")]
pub fn get_user_by_id(id: i32) -> Result<Accepted<Json<String>>, BadRequest<Json<String>>> {
    match get_user_by_id_service::execute(id) {
        Ok(user) => Ok(Accepted(Some(Json(format!("{:#?}", user))))),
        Err(e) => Err(BadRequest(Some(Json(format!("Error: {}", e.to_string()))))),
    }
}
