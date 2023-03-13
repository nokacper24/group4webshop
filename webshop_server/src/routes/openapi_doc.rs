use actix_web::web;
use utoipa::{
    openapi::{self, ContactBuilder, InfoBuilder, OpenApiBuilder},
    OpenApi,
};
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::routes::public::{licenses, products, testimonials, users};

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {
    let info = build_info();

    cfg.service(SwaggerUi::new("/api-doc/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("Products", "/api-doc/openapi_products.json"),
            build_products_doc(info.clone()),
        ),
        (
            Url::new("Users", "/api-doc/openapi_users.json"),
            build_users_doc(info.clone()),
        ),
        (
            Url::new("Licenses", "/api-doc/openapi_licenses.json"),
            build_licenses_doc(info.clone()),
        ),
        (
            Url::new("Testimonials", "/api-doc/openapi_testimonials.json"),
            build_licenses_doc(info.clone()),
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
                .email(Some("admin@proflexdomain.com"))
                .url(Some("https://proflexdomain.com"))
                .build(),
        ))
        .license(Some(
            openapi::LicenseBuilder::new()
                .name("GNU")
                .url(Some("https://www.gnu.org/licenses/gpl-3.0.en.html"))
                .build(),
        ))
        .build()
}

fn build_products_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(products::ProductsApiDoc::openapi())
        .info(info)
        .build()
}

fn build_users_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(users::UserApiDoc::openapi())
        .info(info)
        .build()
}

fn build_licenses_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(licenses::LicensesOpenApi::openapi())
        .info(info)
        .build()
}

fn build_testimonials_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(testimonials::TestimonialsOpenApi::openapi())
        .info(info)
        .build()
}
