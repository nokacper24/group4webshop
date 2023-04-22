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
mod routes;
mod utils;

use routes::private::private;
use routes::public::public;

use crate::data_access::create_pool;
use crate::routes::{openapi_doc, serving_images};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const DEFALUT_LOG_LEVEL: &str = "info,sqlx=warn";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if let Ok(level) = std::env::var("RUST_LOG") {
        if level.trim().is_empty() {
            std::env::set_var("RUST_LOG", DEFALUT_LOG_LEVEL);
        }
    } else {
        std::env::set_var("RUST_LOG", DEFALUT_LOG_LEVEL);
    }
    env_logger::init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
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
        .split(',')
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
            .supports_credentials()
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

    // CERT_PATH=certificate.pem
    // PRIV_KEY_PATH=privatekey.pem

    let cert_path = std::env::var("CERT_PATH").expect("CERT_PATH environment variable not set");
    let priv_key_path =
        std::env::var("PRIV_KEY_PATH").expect("PRIV_KEY_PATH environment variable not set");

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(priv_key_path).unwrap());

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
