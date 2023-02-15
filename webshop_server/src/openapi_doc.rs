use actix_web::web;
use utoipa::{openapi::{self, InfoBuilder, ContactBuilder, OpenApiBuilder, Components, ComponentsBuilder}, OpenApi};
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::{
    data_access::{product::Product, user::User},
    routes::public::{products, users},
};

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {
    let info = InfoBuilder::new()
        .title("My api")
        .version("1.0.0")
        .contact(Some(
            ContactBuilder::new()
                .name(Some("Admin Admin"))
                .email(Some("amdin@petapi.com"))
                .build(),
        ))
        .build();
    
    let productsdoc = OpenApiBuilder::from(ProductsApiDoc::openapi())
        .info(info.clone())
        .build();

    let usersdoc = OpenApiBuilder::from(UserApiDoc::openapi())
        .info(info.clone())
        .build();

    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("Products", "/api-doc/openapi_products.json"),
            productsdoc,
        ),
        (
            Url::new("Users", "/api-doc/openapi_users.json"),
            usersdoc,
        ),
    ]));
}

#[derive(OpenApi)]
#[openapi(
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

#[derive(OpenApi)]
#[openapi(info(
    title = "Webshop API",
    version = "0.1.0",
    description = "API for the webshop",
    license(name = "MIT", url = "https://opensource.org/licenses/MIT",),
    contact(
        name = "Example",
        url = "https://example.com",
        email = "mail@example.com",
    ),
))]
pub struct CommonApiDoc;
