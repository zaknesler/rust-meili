use actix_web::{
    get,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use meilisearch_sdk::{client, search::SearchResults};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[derive(Serialize)]
struct MovieResponse {
    movies: Vec<Movie>,
}

#[derive(Serialize)]
struct ApiMessage {
    message: String,
}

impl From<SearchResults<Movie>> for MovieResponse {
    fn from(results: SearchResults<Movie>) -> Self {
        MovieResponse {
            movies: results
                .hits
                .iter()
                .map(|result| result.result.clone())
                .collect(),
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/movies")]
async fn movies(req: HttpRequest) -> Result<impl Responder, actix_web::error::QueryPayloadError> {
    let query = web::Query::<SearchQuery>::from_query(req.query_string())?;

    // Create a client (without sending any request so that can't fail)
    let client = client::Client::new("http://localhost:7700", "masterKey");

    Ok(web::Json(MovieResponse::from(
        client
            .index("movies")
            .search()
            .with_query(&query.q)
            .execute::<Movie>()
            .await
            .unwrap(),
    )))
}

#[get("/add")]
async fn add() -> impl Responder {
    let client = client::Client::new("http://localhost:7700", "masterKey");

    client
        .index("movies")
        .add_documents(
            &[
                Movie {
                    id: 1,
                    title: String::from("Carol"),
                    genres: vec!["Romance".to_string(), "Drama".to_string()],
                },
                Movie {
                    id: 2,
                    title: String::from("Wonder Woman"),
                    genres: vec!["Action".to_string(), "Adventure".to_string()],
                },
                Movie {
                    id: 3,
                    title: String::from("Life of Pi"),
                    genres: vec!["Adventure".to_string(), "Drama".to_string()],
                },
                Movie {
                    id: 4,
                    title: String::from("Mad Max"),
                    genres: vec!["Adventure".to_string(), "Science Fiction".to_string()],
                },
                Movie {
                    id: 5,
                    title: String::from("Moana"),
                    genres: vec!["Fantasy".to_string(), "Action".to_string()],
                },
                Movie {
                    id: 6,
                    title: String::from("Philadelphia"),
                    genres: vec!["Drama".to_string()],
                },
            ],
            Some("id"),
        )
        .await
        .unwrap();

    web::Json(ApiMessage {
        message: String::from("Movies have been added."),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(movies).service(add))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
