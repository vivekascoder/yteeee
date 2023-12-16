// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Nullable<Text>,
        youtube_url -> Text,
        status -> Text,
    }
}
