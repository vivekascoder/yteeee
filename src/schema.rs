// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        youtube_url -> Text,
        status -> Text,
        result -> Nullable<Text>,
    }
}
