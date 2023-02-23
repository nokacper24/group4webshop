use actix_cors::Cors;
use actix_web::{get, http, web, App, middleware::Logger, HttpServer, Responder, HttpResponse};
use actix_web_static_files::ResourceFiles;
use dotenvy::dotenv;
use log::info;

mod data_access;
mod routes;
mod openapi_doc;

use routes::public::public;

use crate::data_access::create_pool;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,sqlx=off");
    env_logger::init();

    // check if .env file exists and load it
    dotenv().ok();
    let host = std::env::var("HOST").expect("HOST environment variable not set");
    let port = std::env::var("PORT").expect("PORT environment variable not set");
    let address = format!("{}:{}", host, port);

    info!("Starting server at http://{}", address);

    //create new pool
    let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    let pool = web::Data::new(
        create_pool(dburl.as_str())
            .await
            .expect("Can not connect to database"),
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Should be changed to allow only specific origins in production
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let public = web::scope("/api").configure(public);

        App::new()
            // register sqlx pool
            .app_data(pool.clone())
            // configure cors
            .wrap(cors)
            .wrap(Logger::default())
            .service(public)
            .configure(openapi_doc::configure_opanapi)
            .service(ResourceFiles::new("/", generate()).resolve_not_found_to_root())
            .default_service(web::route().to(not_found))
    })
    .bind(address)
    .expect("Can not bind to port 8080")
    .run()
    .await
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body(
        r#"<html>
            <head>
                <title>404 Not Found</title>
            </head>
            <body>
                <h1>404 Not Found</h1>
                <p>The resource could not be found.</p>
            </body>
        </html>"#)
}
