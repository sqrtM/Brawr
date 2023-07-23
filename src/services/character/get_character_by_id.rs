use diesel::{RunQueryDsl, result::Error, QueryDsl, ExpressionMethods};

use crate::{
    models::character::Character, 
    services::pg_connection::establish_connection_pg, 
    schema::characters::{
        self, 
        character_id,
    }
};

pub fn execute(id: i32) -> Result<Vec<Character>, Error> {
    let conn = &mut establish_connection_pg();
    characters::table.filter(character_id.eq(id)).load(conn)
}
