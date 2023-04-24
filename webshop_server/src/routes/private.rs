use actix_web::{get, web, Responder};

pub mod auth;
pub mod companies_protected;
pub mod licenses_protected;
pub mod me;
pub mod products_protected;
pub mod testimonials_protected;
pub mod users_protected;

#[get("")]
async fn index() -> impl Responder {
    "Hello world!"
}

pub fn private(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::configure);
    cfg.configure(products_protected::configure);
    cfg.configure(licenses_protected::configure);
    cfg.configure(me::configure);
    cfg.configure(users_protected::configure);
    cfg.configure(testimonials_protected::configure);
    cfg.configure(companies_protected::configure);
}
