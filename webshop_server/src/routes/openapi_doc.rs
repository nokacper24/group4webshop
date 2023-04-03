use actix_web::web;
use utoipa::{
    openapi::{self, ContactBuilder, InfoBuilder, OpenApiBuilder, Paths},
    OpenApi,
};
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::routes::public::{licenses, products, testimonials};

use super::public::products::descriptions;

fn public_docs() -> Vec<openapi::OpenApi> {
    vec![
        products::ProductsApiDoc::openapi(),
        descriptions::DescriptionApiDoc::openapi(),
        licenses::LicensesOpenApi::openapi(),
        testimonials::TestimonialsOpenApi::openapi(),
    ]
}

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
    let all_pub_docs = public_docs();
    OpenApiBuilder::new()
        .components(extract_components(all_pub_docs.clone()))
        .paths(extract_paths(all_pub_docs.clone()))
        .tags(extract_tags(all_pub_docs))
        .info(info)
        .build()
}

fn extract_paths(docs: Vec<openapi::OpenApi>) -> Paths {
    let mut paths = Paths::new();
    for mut doc in docs {
        paths.paths.append(&mut doc.paths.paths);
    }
    paths
}

fn extract_components(docs: Vec<openapi::OpenApi>) -> Option<openapi::Components> {
    let mut components = openapi::Components::new();
    for doc in docs {
        if let Some(mut doc_components) = doc.components {
            components.schemas.append(&mut doc_components.schemas);
            components.security_schemes
                .append(&mut doc_components.security_schemes);
            components.responses.append(&mut doc_components.responses);
        }
    }
    Some(components)
}

fn extract_tags(docs: Vec<openapi::OpenApi>) -> Option<Vec<utoipa::openapi::Tag>> {
    let mut tags = Vec::new();
    for doc in docs {
        if let Some(mut doc_tags) = doc.tags {
            tags.append(&mut doc_tags);
        }
    }
    Some(tags)
}
