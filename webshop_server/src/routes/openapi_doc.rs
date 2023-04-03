use actix_web::web;
use utoipa::{
    openapi::{self, ContactBuilder, InfoBuilder, OpenApiBuilder, Paths},
    OpenApi,
};
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::routes::public::{licenses, products, testimonials};

use super::public::products::descriptions;

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {
    let info = build_info();

    cfg.service(SwaggerUi::new("/api-doc/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("Public", "/api-doc/public_endpoints.json"),
            build_public_endpoints_doc(info.clone()),
        ),
        (
            Url::new("Products", "/api-doc/openapi_products.json"),
            build_products_doc(info.clone()),
        ),
        (
            Url::new("Descriptions", "/api-doc/openapi_descriptions.json"),
            build_description_doc(info.clone()),
        ),
        (
            Url::new("Licenses", "/api-doc/openapi_licenses.json"),
            build_licenses_doc(info.clone()),
        ),
        (
            Url::new("Testimonials", "/api-doc/openapi_testimonials.json"),
            build_licenses_doc(info),
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

fn build_description_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::from(products::descriptions::DescriptionApiDoc::openapi())
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

fn build_public_endpoints_doc(info: openapi::Info) -> openapi::OpenApi {
    OpenApiBuilder::new()
        .components(build_public_components())
        .paths(build_public_paths())
        .tags(build_public_tags())
        .info(info)
        .build()
}

fn build_public_paths() -> Paths {
    let mut paths = Paths::new();
    paths
        .paths
        .append(&mut products::ProductsApiDoc::openapi().paths.paths);
    paths
        .paths
        .append(&mut descriptions::DescriptionApiDoc::openapi().paths.paths);
    paths
}

fn build_public_components() -> Option<openapi::Components> {
    let mut components = openapi::Components::new();
    if let Some(mut products_components) = products::ProductsApiDoc::openapi().components {
        components.schemas.append(&mut products_components.schemas);
    }
    if let Some(mut descriptions_components) = descriptions::DescriptionApiDoc::openapi().components
    {
        components
            .schemas
            .append(&mut descriptions_components.schemas);
    }
    Some(components)
}

fn build_public_tags() -> Option<Vec<utoipa::openapi::Tag>> {
    let mut tags = Vec::new();
    if let Some(mut products_tags) = products::ProductsApiDoc::openapi().tags {
        tags.append(&mut products_tags);
    }
    if let Some(mut descriptions_tags) = descriptions::DescriptionApiDoc::openapi().tags {
        tags.append(&mut descriptions_tags);
    }
    Some(tags)
}
