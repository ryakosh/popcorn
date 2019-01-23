table! {
    artists (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
    }
}

table! {
    directors (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
    }
}

table! {
    movies (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        genres -> Nullable<Array<Text>>,
        languages -> Nullable<Array<Text>>,
        release_country -> Nullable<Varchar>,
        release_date -> Nullable<Date>,
        duration -> Nullable<Int2>,
        directors -> Nullable<Array<Int4>>,
        writers -> Nullable<Array<Int4>>,
        stars -> Nullable<Array<Int4>>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        pwd -> Varchar,
    }
}

table! {
    writers (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    artists,
    directors,
    movies,
    users,
    writers,
);
