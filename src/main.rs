use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, debug_handler, extract::State, routing::post};

fn main() {
    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload.
    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
    let db = Arc::new(App::new());
    // Post
    let app = Router::new()
        .route("/movie", post(store_movie))
        .with_state(db.clone());
}

struct App {
    db: Database,
}

struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

struct Database {
    movies: HashMap<String, Movie>,
}

impl Database {
    fn new() -> Self {
        Self {
            movies: HashMap::new(),
        }
    }
}

#[debug_handler]
async fn store_movie(Json(payload): Json<Movie>, State(state): State<Arc<App>>) {
    //let movie = serde_json::from_value(payload);
    let movie_name = payload.name.clone();

    let mut s = Arc::clone(&state);
    s.db.movies.insert(movie_name, payload);
}
