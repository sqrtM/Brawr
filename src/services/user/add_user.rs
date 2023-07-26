use chrono::NaiveDate;
use diesel::{query_dsl::methods::FilterDsl, result::Error, ExpressionMethods, RunQueryDsl};
use rocket::serde::json::Json;

use crate::{
    models::{
        session::{NewSession, Session},
        user::{User, UserRequest, UserResponseWithSecret},
    },
    schema::{
        sessions::{self, session_id},
        users as user_schema,
    },
    services::pg_connection::establish_connection_pg,
};

pub fn execute(user: Json<UserRequest>) -> Result<UserResponseWithSecret, String> {
    let new_user = UserRequest {
        email: user.email.to_string(),
        password: user.password.to_string(),
    };
    let new_session = NewSession {
        user_id: 1,
        expires_at: NaiveDate::from_ymd_opt(2023, 7, 27)
            .unwrap()
            .and_hms_opt(0, 0, 3)
            .unwrap(),
        secret: sha256::digest(new_user.password.clone()),
    };
    let conn: &mut diesel::PgConnection = &mut establish_connection_pg();
    let user: Result<Vec<User>, Error> = diesel::insert_into(user_schema::table)
        .values(&new_user)
        .get_results::<User>(conn);
    match user {
        Ok(user) => {
            let se = diesel::insert_into(sessions::table)
                .values(&new_session)
                .execute(conn);
            match se {
                Ok(ses) => {
                    let selected_session: Result<Session, Error> = sessions::table
                        .filter(session_id.eq(ses as i32))
                        .first::<Session>(conn);
                    match selected_session {
                        Ok(d) => {
                            let secret = d.secret;
                            Ok(UserResponseWithSecret {
                                user,
                                secret,
                            })
                        }
                        Err(_e) => Err("No".to_string()),
                    }
                }
                Err(_) => Err("No".to_string()),
            }
        }
        Err(_) => Err("No".to_string()),
    }
}
