use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::{routes::public::{products, users}, data_access::{product::Product, user::User}};

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .urls(vec![
                (Url::new("Products", "/api-doc/openapi_products.json"), ProductsApiDoc::openapi()),
                (Url::new("Users", "/api-doc/openapi_users.json"), UserApiDoc::openapi()),
                ]
            ),
    );
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Webshop API",
        version = "0.1.0",
        description = "API for the webshop",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT",),
        contact(
            name = "Example",
            url = "https://example.com",
            email = "mail@example.com",),),
    paths(
        products::products,
        products::product_by_id,
    ),
    components(
        schemas(Product)
    ),
    tags(
        (name = "Products", description = "Api endpoints for products")
    ),
)]
pub struct ProductsApiDoc;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Webshop API",
        version = "0.1.0",
        description = "API for the webshop",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT",),
        contact(
            name = "Example",
            url = "https://example.com",
            email = "mail@example.com",),),
    paths(
        users::users,
    ),
    components(
        schemas(User)
    ),
    tags(
        (name = "Users", description = "Api endpoints for users")
    ),
)]
pub struct UserApiDoc;