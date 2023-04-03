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

fn protected_docs() -> Vec<openapi::OpenApi> {
    vec![
        // nothing here yet
    ]
}

fn admin_docs() -> Vec<openapi::OpenApi> {
    vec![
        // nothing here yet
    ]
}

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {
    cfg.service(SwaggerUi::new("/api-doc/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("Public", "/api-doc/public_endpoints.json"),
            build_docs(PrivacyLevel::Public),
        ),
        (
            Url::new("Protected", "/api-doc/openapi_products.json"),
            build_docs(PrivacyLevel::Protected),
        ),
        (
            Url::new("Admin", "/api-doc/openapi_descriptions.json"),
            build_docs(PrivacyLevel::Admin),
        ),
    ]));
}

fn build_info(level: PrivacyLevel) -> openapi::Info {
    let description = match level {
        PrivacyLevel::Public => "Public endpoints. No authentication required.",
        PrivacyLevel::Protected => "Protected endpoints. Authentication required.",
        PrivacyLevel::Admin => "Admin endpoints. Authentication required, with admin privileges.",
    };
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
        .description(Some(description.to_string()))
        .build()
}

enum PrivacyLevel {
    Public,
    Protected,
    Admin,
}

fn build_docs(level: PrivacyLevel) -> openapi::OpenApi {
    let all_pub_docs = match level {
        PrivacyLevel::Public => public_docs(),
        PrivacyLevel::Protected => protected_docs(),
        PrivacyLevel::Admin => admin_docs(),
    };
    let info = build_info(level);

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
            components
                .security_schemes
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
