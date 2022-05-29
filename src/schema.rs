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
joinable!(playings -> users (discoverer));

allow_tables_to_appear_in_same_query!(
    eatings,
    playings,
    users,
);
