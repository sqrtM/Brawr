use diesel::{result::Error, RunQueryDsl};
use rocket::serde::json::Json;

use crate::{
    models::character::{
        Character, 
        NewCharacterRequest
    },
    schema::characters as character_schema,
    services::pg_connection::establish_connection_pg,
};

pub fn execute(character: Json<NewCharacterRequest>) -> Result<Vec<Character>, Error> {
    let new_character = NewCharacterRequest {
        user_id: character.user_id,
        character_name: character.character_name.to_string(),
        constitution: character.constitution,
        strength: character.strength,
        madness: character.madness,
        intelligence: character.intelligence,
    };
    let conn: &mut diesel::PgConnection = &mut establish_connection_pg();
    diesel::insert_into(character_schema::table)
        .values(&new_character)
        .get_results(conn)
}
