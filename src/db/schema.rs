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
    users_favorites (user_id, movie_id) {
        user_id -> Varchar,
        movie_id -> Int4,
    }
}

table! {
    users_ratings (user_id, movie_id) {
        user_id -> Varchar,
        movie_id -> Int4,
        user_rating -> Int2,
    }
}

table! {
    users_watchlist (user_id, movie_id) {
        user_id -> Varchar,
        movie_id -> Int4,
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

joinable!(users_favorites -> movies (movie_id));
joinable!(users_favorites -> users (user_id));
joinable!(users_watchlist -> movies (movie_id));
joinable!(users_watchlist -> users (user_id));

allow_tables_to_appear_in_same_query!(
    artists,
    directors,
    movies,
    movies_artists,
    movies_directors,
    movies_writers,
    users,
    users_favorites,
    users_ratings,
    users_watchlist,
    writers,
);
