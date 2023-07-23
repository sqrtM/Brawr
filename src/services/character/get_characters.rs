use diesel::{RunQueryDsl, result::Error, QueryDsl};

use crate::{
    models::character::Character, 
    services::pg_connection::establish_connection_pg, 
    schema::characters::{
        self, 
        character_id
    }
};

pub fn execute() -> Result<Vec<Character>, Error> {
    let conn = &mut establish_connection_pg();
    characters::table.order(character_id).load(conn)
}
