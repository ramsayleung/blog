table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Varchar,
        raw_content -> Text,
        rendered_content -> Text,
        create_time -> Timestamp,
        modify_time -> Timestamp,
        post_type -> Int4,
        hit_time -> Int4,
        published -> Bool,
        slug_url -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        hashed_password -> Varchar,
        create_time -> Timestamp,
        modify_time -> Timestamp,
        email -> Varchar,
        avatar_url -> Nullable<Varchar>,
    }
}

table! {
    visitor_log (id) {
        id -> Int4,
        ip -> Inet,
        access_time -> Timestamp,
        user_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    post,
    user,
    visitor_log,
);
