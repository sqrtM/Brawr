// @generated automatically by Diesel CLI.

diesel::table! {
    characters (character_id) {
        character_id -> Int4,
        user_id -> Int4,
        #[max_length = 70]
        character_name -> Varchar,
        constitution -> Int4,
        strength -> Int4,
        madness -> Int4,
        intelligence -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sessions (session_id) {
        session_id -> Int4,
        user_id -> Int4,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        #[max_length = 64]
        secret -> Bpchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 72]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(characters -> users (character_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    sessions,
    users,
);
