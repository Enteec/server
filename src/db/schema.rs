// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        salt -> Text,
    }
}
