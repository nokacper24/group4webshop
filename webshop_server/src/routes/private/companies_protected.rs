use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    data_access::{company, error_handling},
    SharedData,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(companies);
}

/// Get all companies from the database.
/// returns a json array of all companies.
#[get("/companies")]
async fn companies(shared_data: web::Data<SharedData>) -> impl Responder {
    let pool = &shared_data.db_pool;
    let companies = company::get_all_companies(&pool).await;

    match companies {
        Ok(companies) => HttpResponse::Ok().json(companies),
        Err(e) => match e {
            sqlx::Error::Database(e) => match error_handling::PostgresDBError::from_str(e) {
                error_handling::PostgresDBError::NoDataFound => {
                    HttpResponse::NotFound().json("No companies found")
                }
                // if the data already exists, return a Conflict
                error_handling::PostgresDBError::UniqueViolation => {
                    HttpResponse::Conflict().json("Company already exists")
                }
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            },
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        },
    }
}
