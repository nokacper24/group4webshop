use actix_cors::Cors;
use actix_web::{
    get, http, http::Error, middleware::Logger, web, web::ReqData, App, HttpResponse, HttpServer,
    Responder,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_static_files::ResourceFiles;
use dotenvy::dotenv;
use log::info;

mod data_access;
mod middlewares;
mod routes;

use routes::private::private;
use routes::public::public;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::data_access::create_pool;
use crate::data_access::user::Role;
use crate::middlewares::auth::{check_role, validator, Token};
use crate::routes::{openapi_doc, serving_images};

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
        let cors = Cors::permissive();

        let bearer_middleware = HttpAuthentication::bearer(validator);

        // public enpoints at `/api`, private endpoints at `/api/priv`
        let api_endpoints = web::scope("/api").configure(public).service(
            web::scope("/priv")
                .wrap(bearer_middleware)
                .configure(private),
        );

        App::new()
            // register sqlx pool
            .app_data(pool.clone())
            // configure cors
            .wrap(cors)
            .wrap(Logger::default())
            .service(api_endpoints)
            .configure(openapi_doc::configure_opanapi)
            .service(web::scope("/resources/images").configure(serving_images::config))
            .service(ResourceFiles::new("/", generate()).resolve_not_found_to_root())
            .default_service(web::route().to(not_found))
    })
    .bind(address)
    .expect("Can not bind to port 8080")
    .run()
    .await
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body(r#"<html>
    <head>
        <title>404 Not Found</title>
    </head>
    <body>
        <h1>404 Not Found</h1>
        <p>The resource could not be found.</p>
    </body>
</html>"#,
    )
}
