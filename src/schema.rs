// @generated automatically by Diesel CLI.

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
