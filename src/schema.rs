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

joinable!(posts -> users (user_id));
joinable!(profiles -> users (id));
joinable!(tags -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    profiles,
    tags,
    users,
);
