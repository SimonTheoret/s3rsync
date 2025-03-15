// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Integer,
        title -> Text,
        path -> Text,
        date -> Integer,
    }
}
