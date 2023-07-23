use diesel::{result::Error, BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{
        character::Character,
        user::{User, UserResponse},
    },
    schema::users::{self, user_id},
    services::pg_connection::establish_connection_pg,
};

pub fn execute(id: i32) -> Result<UserResponse, Error> {
    let conn: &mut diesel::PgConnection = &mut establish_connection_pg();
    match users::table.filter(user_id.eq(id)).first::<User>(conn) {
        Ok(selected_user) => { 
            let user_characters = Character::belonging_to(&selected_user).load::<Character>(conn)?;
            Ok(UserResponse {
                user: selected_user,
                characters: user_characters,
            })
        },
        Err(e) => Err(e)
    }
}
