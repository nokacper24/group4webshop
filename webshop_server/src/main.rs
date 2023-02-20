use actix_cors::Cors;
use actix_web::HttpResponse;
use actix_web::http::Error;
use actix_web::web::ReqData;
use actix_web::{get, http, middleware::Logger, web, App, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenvy::dotenv;
use log::info;

mod data_access;
mod middlewares;
mod openapi_doc;
mod routes;

use routes::public::public;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

use crate::data_access::create_pool;
use crate::data_access::user::Role;
use crate::middlewares::auth::{validator, check_role, Token};





#[get("/")]
async fn index(req_token: Option<ReqData<Token>>, pool: web::Data<Pool<Postgres>> )  -> impl Responder {
    let role = checkRole(req_token, pool).await;
    match role {
        Ok(role) => {
            match role {
                Role::Admin => HttpResponse::Ok().body("Hello Admin"),
                Role::CompanyItHead => HttpResponse::Ok().body("Hello Company IT Head"),
                Role::CompanyIt => HttpResponse::Ok().body("Hello Company IT"),
                Role::Default => HttpResponse::Ok().body("Hello Default"),
            }
        }
        Err(e) => HttpResponse::Unauthorized().body(e),
    }
}

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

        let bearer_middleware = HttpAuthentication::bearer(validator);

        App::new()
            // register sqlx pool
            .app_data(pool.clone())
            // configure cors
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(bearer_middleware)
            
            .service(index)
            // load routes from routes/public/public.rs
            .service(public)
            .configure(openapi_doc::configure_opanapi)
            // Configure custom 404 page
            .default_service(web::route().to(|| async { "404 - Not Found" }))
    })
    .bind(address)
    .expect("Can not bind to port 8080")
    .run()
    .await
}
