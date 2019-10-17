table! {
    artists (artist_id) {
        artist_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
    }
}

table! {
    directors (director_id) {
        director_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
    }
}

table! {
    movies (movie_id) {
        movie_id -> Int4,
        title -> Varchar,
        description -> Text,
        poster -> Varchar,
        genres -> Array<Text>,
        languages -> Array<Text>,
        release_country -> Varchar,
        release_date -> Date,
        duration -> Int2,
        rating -> Float4,
        rating_count -> Int4,
    }
}

table! {
    movies_artists (movie_id, artist_id) {
        movie_id -> Int4,
        artist_id -> Int4,
    }
}

table! {
    movies_directors (movie_id, director_id) {
        movie_id -> Int4,
        director_id -> Int4,
    }
}

table! {
    movies_writers (movie_id, writer_id) {
        movie_id -> Int4,
        writer_id -> Int4,
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
    users_ratings (user_id, movie_id) {
        user_id -> Int4,
        movie_id -> Int4,
        user_rating -> Int2,
    }
}

table! {
    writers (writer_id) {
        writer_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    artists,
    directors,
    movies,
    movies_artists,
    movies_directors,
    movies_writers,
    users,
    users_ratings,
    writers,
);
