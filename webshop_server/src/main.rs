use std::fs::File;
use std::io::BufReader;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};

use actix_web_static_files::ResourceFiles;
use dotenvy::dotenv;
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
        let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .expect("ALLOWED_ORIGINS environment variable not set. Ex: http://localhost:8080,http://localhost:8081")
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
            .allowed_origin_fn(move |origin, _req_head| {
                allowed_origins.iter().any(|allowed| allowed == origin)
            });
        let api_endpoints = web::scope("/api")
            .configure(public)
            .service(web::scope("/priv").configure(private));

        let image_service = web::scope("/resources/images").configure(serving_images::config);
        let static_files = ResourceFiles::new("/", generate()).resolve_not_found_to_root();
        App::new()
            .app_data(pool.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .service(api_endpoints)
            .configure(openapi_doc::configure_opanapi)
            .service(image_service)
            .service(static_files)
            .default_service(web::route().to(routes::not_found))
    });

    match server.bind_rustls(address.clone(), tls_config) {
        Ok(server) => server.run().await,
        Err(e) => {
            eprintln!("Could not bind to address: {}", address);
            panic!("Error: {}", e);
        }
    }
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
