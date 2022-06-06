table! {
    eatings (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        discoverer -> Int4,
        create_on -> Timestamp,
        update_on -> Timestamp,
    }
}

table! {
    eatings_uploads (id) {
        id -> Int4,
        eating_id -> Int4,
        upload_id -> Int4,
    }
}

table! {
    playings (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        discoverer -> Int4,
        create_on -> Timestamp,
        update_on -> Timestamp,
    }
}

table! {
    playings_uploads (id) {
        id -> Int4,
        playing_id -> Int4,
        upload_id -> Int4,
    }
}

table! {
    uploads (id) {
        id -> Int4,
        fetch_code -> Varchar,
        owner -> Int4,
        create_on -> Timestamp,
        update_on -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        create_on -> Timestamp,
        update_on -> Timestamp,
    }
}

joinable!(eatings -> users (discoverer));
joinable!(eatings_uploads -> eatings (eating_id));
joinable!(eatings_uploads -> uploads (upload_id));
joinable!(playings -> users (discoverer));
joinable!(playings_uploads -> playings (playing_id));
joinable!(playings_uploads -> uploads (upload_id));
joinable!(uploads -> users (owner));

allow_tables_to_appear_in_same_query!(
    eatings,
    eatings_uploads,
    playings,
    playings_uploads,
    uploads,
    users,
);
