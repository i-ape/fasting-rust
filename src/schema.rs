// @generated automatically by Diesel CLI.

diesel::table! {
    fasting_events (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        start_time -> Timestamp,
        stop_time -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    fasting_goals (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        goal_duration -> Integer,
        deadline -> Timestamp,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    fasting_sessions (id) {
        id -> Integer,
        user_id -> Integer,
        start_time -> Timestamp,
        stop_time -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        hashed_password -> Text,
        device_id -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(fasting_events -> users (user_id));
diesel::joinable!(fasting_goals -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    fasting_events,
    fasting_goals,
    fasting_sessions,
    users,
);
