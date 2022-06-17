table! {
    comments (id) {
        id -> Int4,
        rank -> Int4,
        content -> Text,
        user -> Int4,
        location -> Int4,
        create_on -> Timestamp,
        update_on -> Timestamp,
    }
}

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
    equipments (id) {
        id -> Int4,
        name -> Varchar,
        is_required -> Bool,
        usage -> Text,
        location -> Int4,
        create_on -> Timestamp,
        update_on -> Timestamp,
    }
}

table! {
    location_upload_rels (id) {
        id -> Int4,
        location_id -> Int4,
        upload_id -> Int4,
    }
}

table! {
    locations (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        category -> Int4,
        description -> Text,
        discoverer -> Int4,
        create_on -> Timestamp,
        update_on -> Timestamp,
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

joinable!(comments -> locations (location));
joinable!(comments -> users (user));
joinable!(eatings -> users (discoverer));
joinable!(eatings_uploads -> eatings (eating_id));
joinable!(eatings_uploads -> uploads (upload_id));
joinable!(equipments -> locations (location));
joinable!(location_upload_rels -> locations (location_id));
joinable!(location_upload_rels -> uploads (upload_id));
joinable!(locations -> users (discoverer));
joinable!(playings -> users (discoverer));
joinable!(playings_uploads -> playings (playing_id));
joinable!(playings_uploads -> uploads (upload_id));
joinable!(uploads -> users (owner));

allow_tables_to_appear_in_same_query!(
    comments,
    eatings,
    eatings_uploads,
    equipments,
    location_upload_rels,
    locations,
    playings,
    playings_uploads,
    uploads,
    users,
);
