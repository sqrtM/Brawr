use diesel::{RunQueryDsl, result::Error, QueryDsl};

use crate::{
    models::user::User, 
    services::pg_connection::establish_connection_pg, 
    schema::users::{
        self, 
        user_id
    }
};

pub fn execute() -> Result<Vec<User>, Error> {
    let conn = &mut establish_connection_pg();
    users::table.order(user_id).load(conn)
}
