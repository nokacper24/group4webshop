use actix_web::web;
use utoipa::{openapi::{self, InfoBuilder, ContactBuilder, OpenApiBuilder, Components, ComponentsBuilder}, OpenApi};
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::{
    data_access::{product::Product, user::User},
    routes::public::{products, users},
};

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {

    let info = build_info();    

    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("Products", "/api-doc/openapi_products.json"),
            build_products_doc(info.clone()),
        ),
        (
            Url::new("Users", "/api-doc/openapi_users.json"),
            build_users_doc(info.clone()),
        ),
    ]));
}

fn build_info() -> openapi::Info {
    InfoBuilder::new()
    .title("ProFlex API")
        .version("1.0.0")
        .contact(Some(
            ContactBuilder::new()
                .name(Some("ProFlex"))
                .email(Some("amdin@proflexdomain.com"))
                .url(Some("https://proflexdomain.com"))
                .build(),
        ))
        .license(
            Some(
                openapi::LicenseBuilder::new()
                    .name("GNU")
                    .url(Some("https://www.gnu.org/licenses/gpl-3.0.en.html"))
                    .build(),
            ),
        )
        .build()
}

fn build_products_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(ProductsApiDoc::openapi())
        .info(info)
        .build()
}

fn build_users_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(UserApiDoc::openapi())
        .info(info)
        .build()
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
