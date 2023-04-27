use core::panic;
use std::fs::File;
use std::io::BufReader;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http, middleware::Logger, web, App, HttpServer};

use actix_web_static_files::ResourceFiles;
use dotenvy::dotenv;
use flexi_logger::{Duplicate, FileSpec};
use log::info;
use rustls::{self, Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

mod data_access;
mod routes;
mod utils;

use routes::private::private;
use routes::public::public;

use crate::data_access::create_pool;
use crate::routes::{openapi_doc, serving_images};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const DEFALUT_LOG_LEVEL: &str = "info,sqlx=warn";

/// Data shared between actix-web threads.
#[derive(Clone)]
struct SharedData {
    /// The database pool.
    db_pool: sqlx::PgPool,
    /// The mailer for sending emails.
    mailer: SmtpTransport,
}

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
    let _logger = match setup_logger().start() {
        Ok(logger) => Some(logger),
        Err(e) => {
            eprintln!("Could not start logger: {}", e);
            None // continue without logging
        }
    };

    let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);
    info!("Starting server at https://{}", address);

    let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let pool = web::Data::new(
        create_pool(dburl.as_str())
            .await
            .expect("Cannot connect to database"),
    );
    let email_usr = std::env::var("EMAIL_USR").expect("EMAIL_USR environment variable not set");
    let email_wpd = std::env::var("EMAIL_PWD").expect("EMAIL_PWD environment variable not set");
    let gmail_creds = Credentials::new(email_usr, email_wpd);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .expect("Could not connect to gmail!")
        .credentials(gmail_creds)
        .build();

    let shared_data = Data::new(SharedData {
        db_pool: pool.get_ref().clone(),
        mailer: mailer.clone(),
    });

    let tls_config = load_rustls_config();
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .expect("ALLOWED_ORIGINS environment variable not set. Ex: ALLOWED_ORIGINS=http://localhost:8080,http://localhost:8083")
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let server = HttpServer::new(move || {
        let allowed_origins = allowed_origins.clone();
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
            .app_data(shared_data.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .service(api_endpoints)
            .configure(openapi_doc::configure_opanapi)
            .service(image_service)
            .service(static_files)
            .default_service(web::route().to(routes::resource_not_found))
    });

    match server.bind_rustls(address.clone(), tls_config) {
        Ok(server) => server.run().await,
        Err(e) => {
            eprintln!("Could not bind to address: {}", address);
            panic!("Error: {}", e);
        }
    }
}

/// Loads the TLS configuration.
/// Paths to the certificate and private key are read from the environment variables `CERT_PATH` and `PRIV_KEY_PATH`.
/// 
/// # Panics
/// Panics if the environment variables are not set or if the certificate or private key could not be loaded.
fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_path = std::env::var("CERT_PATH").expect("CERT_PATH environment variable not set");
    let priv_key_path =
        std::env::var("PRIV_KEY_PATH").expect("PRIV_KEY_PATH environment variable not set");

    let cert_file = &mut BufReader::new(match File::open(cert_path) {
        Ok(file) => file,
        Err(e) => panic!("Could not open certificate file: {}", e),
    });
    let key_file = &mut BufReader::new(match File::open(priv_key_path) {
        Ok(file) => file,
        Err(e) => panic!("Could not open private key file: {}", e),
    });

    
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

/// Sets up the logger.
/// When calling .start(), assign the returned logger to a variable, otherwise it will be dropped and will error out.
fn setup_logger() -> flexi_logger::Logger {
    let logger = match flexi_logger::Logger::try_with_env() {
        Ok(logger) => logger,
        Err(e) => {
            panic!("Could not parse RUST_LOG: {}", e);
        }
    };
    logger
        .log_to_file(FileSpec::default().directory("logs"))
        .write_mode(flexi_logger::WriteMode::Async)
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepCompressedFiles(30),
        )
        .create_symlink("latest.log")
        .duplicate_to_stdout(Duplicate::All)
        .append()
        .format_for_files(flexi_logger::detailed_format)
        .format_for_stdout(flexi_logger::colored_default_format)
        .set_palette("196;208;10;6;8".to_owned())
}
