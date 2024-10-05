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
        hashed_password -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(fasting_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    fasting_sessions,
    users,
);


table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

table! {
    fasting_sessions (id) {
        id -> Integer,
        user_id -> Integer,
        started_at -> Timestamp,
        stopped_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    fasting_sessions,
);