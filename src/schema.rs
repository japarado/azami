table! {
    post_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        user_id -> Int4,
    }
}

table! {
    profiles (id) {
        id -> Int4,
        bio -> Text,
        birthday -> Date,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(post_tags -> posts (post_id));
joinable!(post_tags -> tags (tag_id));
joinable!(posts -> users (user_id));
joinable!(profiles -> users (id));
joinable!(tags -> users (user_id));

allow_tables_to_appear_in_same_query!(
    post_tags,
    posts,
    profiles,
    tags,
    users,
);
