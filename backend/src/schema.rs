table! {
    doctors (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        sex -> Bool,
        year -> Int4,
        education -> Int4,
        occupation -> Int4,
        occupation2 -> Varchar,
        occupation_spec -> Varchar,
        certificate -> Int4,
        modalitet -> Int4,
        modalitet2 -> Varchar,
        phone -> Varchar,
        zoom -> Varchar,
        confirmed -> Bool,
        dateupdated -> Int8,
        user_id -> Int4,
    }
}

table! {
    evaluations (id) {
        id -> Int4,
        sex -> Int4,
        age -> Int4,
        problem -> Varchar,
        help -> Varchar,
        success -> Bool,
        forward -> Bool,
        comment -> Varchar,
        timeslot_id -> Int4,
    }
}

table! {
    timeslots (id) {
        id -> Int4,
        status -> Int4,
        datetime -> Int8,
        doctor_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        role -> Int4,
    }
}

joinable!(doctors -> users (user_id));
joinable!(evaluations -> timeslots (timeslot_id));
joinable!(timeslots -> doctors (doctor_id));

allow_tables_to_appear_in_same_query!(
    doctors,
    evaluations,
    timeslots,
    users,
);
