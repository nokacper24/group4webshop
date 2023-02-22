use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

use crate::data_access::company;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(companies);
}

/// Get all companies from the database.
/// returns a json array of all companies.
#[get("/companies")]
async fn companies(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let companies = company::get_all_companies(&pool).await;

    //error check
    if companies.is_err() {
        return HttpResponse::InternalServerError().json("Internal Server Error");
    }

    //parse to json
    if let Ok(companies) = companies {
        return HttpResponse::Ok().json(companies);
    }

    HttpResponse::InternalServerError().json("Internal Server Error")
}


