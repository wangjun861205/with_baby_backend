table! {
    eatings (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Numeric,
        longitude -> Numeric,
        discoverer -> Int4,
        create_on -> Nullable<Timestamp>,
        update_on -> Nullable<Timestamp>,
    }
}

table! {
    playings (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Numeric,
        longitude -> Numeric,
        discoverer -> Int4,
        create_on -> Nullable<Timestamp>,
        update_on -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
        pasword -> Varchar,
        salt -> Varchar,
        create_on -> Nullable<Timestamp>,
        update_on -> Nullable<Timestamp>,
    }
}

joinable!(eatings -> users (discoverer));
joinable!(playings -> users (discoverer));

allow_tables_to_appear_in_same_query!(
    eatings,
    playings,
    users,
);
