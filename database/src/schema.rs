// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 100]
        realname -> Varchar,
        #[max_length = 100]
        nickname -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Varchar,
        created_at -> Timestamptz,
    }
}
