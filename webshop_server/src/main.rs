use std::fs::File;
use std::io::BufReader;

use actix_cors::Cors;
use actix_web::dev::Service;
use actix_web::{
    get, http, http::Error, middleware::Logger, web, web::ReqData, App, HttpResponse, HttpServer,
    Responder,
};

use actix_web_static_files::ResourceFiles;
use dotenvy::dotenv;
use futures::TryFutureExt;
use log::info;
use rustls::{self, Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

mod data_access;
mod middlewares;
mod routes;

use routes::private::private;
use routes::public::public;

use crate::data_access::create_pool;
use crate::routes::{openapi_doc, serving_images};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,sqlx=off");
    env_logger::init();

    dotenv().ok();
    let host = std::env::var("HOST").unwrap_or("localhost".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    info!("Starting server at https://{}", address);

    //create new pool
    let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    let pool = web::Data::new(
        create_pool(dburl.as_str())
            .await
            .expect("Cannot connect to database"),
    );
    let tls_config = load_rustls_config();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://127.0.0.1:8089")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        // public enpoints at `/api`, private endpoints at `/api/priv`
        let api_endpoints = web::scope("/api")
            .configure(public)
            .service(web::scope("/priv").configure(private));

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
    });

    match server
    .bind_rustls(address.clone(), tls_config) {
        Ok(server) => server.run().await,
        Err(e) => {
            eprintln!("Could not bind to address: {}", address);
            panic!("Error: {}", e);
        }
    }
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
</html>"#,
    )
}
fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("certificate.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("privatekey.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
