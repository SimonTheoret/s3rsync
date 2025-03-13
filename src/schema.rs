// @generated automatically by Diesel CLI.

diesel::table! {
    documents (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        date -> Integer,
    }
}
