use rocket::serde::json::Json;

use crate::{
    models::character::NewCharacterRequest,
    services::character::add_character as add_character_service,
    services::character::get_characters as get_characters_service,
    services::character::get_character_by_id as get_character_by_id_service,
};

#[post("/", data = "<request>")]
pub fn add_character(request: Json<NewCharacterRequest>) -> String {
    match add_character_service::execute(request) {
        Ok(e) => format!("{:#?}", e),
        Err(_) => "error!!!!".to_owned(),
    }
}

#[get("/")]
pub fn get_characters() -> String {
    match get_characters_service::execute() {
        Ok(characters) => format!("{:#?}", characters),
        Err(_) => "error!!!!".to_owned(),
    }
}

#[get("/<id>")]
pub fn get_character_by_id(id: i32) -> String {
    match get_character_by_id_service::execute(id) {
        Ok(characters) => format!("{:#?}", characters),
        Err(_) => "error!!!!".to_owned(),
    }
}
