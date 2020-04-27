table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        content -> Varchar,
        status -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        project_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

joinable!(projects -> users (user_id));
joinable!(tasks -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    users,
);
