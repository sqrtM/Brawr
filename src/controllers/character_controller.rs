use rocket::serde::json::Json;

use crate::{
    models::character::NewCharacterRequest,
    services::character::add_character as add_character_service,
};

#[post("/", data = "<request>")]
pub fn add_character(request: Json<NewCharacterRequest>) -> String {
    match add_character_service::execute(request) {
        Ok(e) => format!("{:#?}", e),
        Err(_) => "error!!!!".to_owned(),
    }
}
