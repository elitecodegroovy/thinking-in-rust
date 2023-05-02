// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        content -> Text,
        created_on -> Timestamp,
        corresponding_question -> Nullable<Int4>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    questions (id) {
        id -> Int8,
        title -> Varchar,
        content -> Text,
        tags -> Nullable<Array<Nullable<Text>>>,
        created_on -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    w_model_category (id) {
        id -> Int8,
        parent_id -> Int8,
        name -> Varchar,
        code -> Varchar,
        level -> Nullable<Int4>,
        icon -> Nullable<Varchar>,
        order_seq -> Nullable<Int4>,
        created_by -> Varchar,
        created_time -> Timestamp,
        status -> Nullable<Int4>,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    answers,
    posts,
    questions,
    users,
    w_model_category,
);
