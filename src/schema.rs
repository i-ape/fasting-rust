// @generated automatically by Diesel CLI.

diesel::table! {
    fasting_sessions (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(fasting_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    fasting_sessions,
    users,
);
