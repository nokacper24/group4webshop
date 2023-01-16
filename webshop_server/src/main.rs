use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpServer, Responder};
use dotenvy::dotenv;

mod routes;

use routes::public::public;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // check if .env file exists and load it
    dotenv().ok();
    let host = std::env::var("HOST").expect("HOST environment variable not set");
    let port = std::env::var("PORT").expect("PORT environment variable not set");
    let address = format!("{}:{}", host, port);

    println!("Starting server at http://{}", address);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin() // Should be changed to allow only specific origins in production
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let public = web::scope("/api").configure(public);

        App::new()
            // configure cors
            .wrap(cors)
            .service(index)
            // load routes from routes/public/public.rs
            .service(public)
            // Configure custom 404 page
            .default_service(web::route().to(|| async { "404 - Not Found" }))
    })
    .bind(address)
    .expect("Can not bind to port 8080")
    .run()
    .await
}
