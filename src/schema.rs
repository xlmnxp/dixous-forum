// @generated automatically by Diesel CLI.

#[cfg(feature = "server")]
diesel::table! {
    threads (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
    }
}
