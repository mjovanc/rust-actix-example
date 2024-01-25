// @generated automatically by Diesel CLI.

diesel::table! {
    categories (category_id) {
        category_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    post_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        user_id -> Int4,
        category_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    profiles (profile_id) {
        profile_id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 100]
        full_name -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        #[max_length = 255]
        website_url -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    tags (tag_id) {
        tag_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password_hash -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    post_tags,
    posts,
    profiles,
    tags,
    users,
);
