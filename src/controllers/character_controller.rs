use rocket::serde::json::Json;

use crate::{
    models::character::NewCharacterRequest,
    services::character::add_character as add_character_service,
    services::character::get_character_by_id as get_character_by_id_service,
    services::character::get_characters as get_characters_service,
};

#[post("/", data = "<request>")]
pub fn add_character(request: Json<NewCharacterRequest>) -> Json<String> {
    match add_character_service::execute(request) {
        Ok(e) => Json(format!("{:#?}", e)),
        Err(e) => Json(format!("Error: {}", e.to_string())),
    }
}

#[get("/")]
pub fn get_characters() -> Json<String> {
    match get_characters_service::execute() {
        Ok(characters) => Json(format!("{:#?}", characters)),
        Err(e) => Json(format!("Error: {}", e.to_string())),
    }
}

#[get("/<id>")]
pub fn get_character_by_id(id: i32) -> Json<String> {
    match get_character_by_id_service::execute(id) {
        Ok(characters) => Json(format!("{:#?}", characters)),
        Err(e) => Json(format!("Error: {}", e.to_string())),
    }
}
