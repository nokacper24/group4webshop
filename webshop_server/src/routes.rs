use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub mod openapi_doc;
pub mod private;
pub mod public;
pub mod serving_images;

/// Default response for 404 Not Found.
///
/// Json response with error and message:
/// ```json
/// {
///    "error": "Not Found",
///   "message": "The resource could not be found."
/// }
/// ```
pub async fn resource_not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "error": "Not Found",
        "message": "The resource could not be found."
    }))
}
