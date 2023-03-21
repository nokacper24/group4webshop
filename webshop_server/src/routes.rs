use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub mod public;
pub mod private;
pub mod openapi_doc;
pub mod serving_images;

pub async fn not_found() -> impl Responder {
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

pub async fn api_not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "error": "Not Found",
        "message": "The resource could not be found."
    }))
}