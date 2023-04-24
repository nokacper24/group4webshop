use crate::routes::{private, public};
use actix_web::web;
use utoipa::{
    openapi::{self, ContactBuilder, InfoBuilder, OpenApiBuilder, Paths},
    OpenApi,
};
use utoipa_swagger_ui::{SwaggerUi, Url};

fn public_routes_docs() -> Vec<openapi::OpenApi> {
    vec![
        public::products::ProductsApiDoc::openapi(),
        public::products::descriptions::DescriptionApiDoc::openapi(),
        public::testimonials::TestimonialsOpenApi::openapi(),
    ]
}

fn protected_routes_docs() -> Vec<openapi::OpenApi> {
    vec![
        // nothing here yet
    ]
}

fn admin_routes_docs() -> Vec<openapi::OpenApi> {
    vec![
        private::products_protected::ProductsApiDoc::openapi(),
        private::products_protected::descriptions_protected::DescriptionApiDoc::openapi(),
        private::testimonials_protected::TestimonialsProtectedOpenApi::openapi(),
    ]
}

pub fn configure_opanapi(cfg: &mut web::ServiceConfig) {
    cfg.service(SwaggerUi::new("/api-doc/swagger-ui/{_:.*}").urls(vec![
        (
            Url::new("Public", "/api-doc/public_endpoints.json"),
            build_docs(PrivacyLevel::Public),
        ),
        (
            Url::new("Protected", "/api-doc/protected_endpoints.json"),
            build_docs(PrivacyLevel::Protected),
        ),
        (
            Url::new("Admin", "/api-doc/admin_endpoints.json"),
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
                .email(Some("admin_proflex@gmail.com"))
                .url(Some("/"))
                .build(),
        ))
        .license(Some(
            openapi::LicenseBuilder::new()
                .name("GNU")
                .url(Some("https://www.gnu.org/licenses/gpl-3.0.en.html"))
                .build(),
        ))
        .description(Some(description.to_string()))
        .terms_of_service(Some("/terms-of-service".to_string()))
        .build()
}

/// The privacy level of the endpoints.
enum PrivacyLevel {
    /// Public endpoints. No authentication required.
    Public,
    /// Protected endpoints. Authentication required.
    Protected,
    /// Admin endpoints. Authentication required, with admin privileges.
    Admin,
}

fn build_docs(level: PrivacyLevel) -> openapi::OpenApi {
    let all_docs = match level {
        PrivacyLevel::Public => public_routes_docs(),
        PrivacyLevel::Protected => protected_routes_docs(),
        PrivacyLevel::Admin => admin_routes_docs(),
    };
    let info = build_info(level);

    OpenApiBuilder::new()
        .components(extract_components(all_docs.clone()))
        .paths(extract_paths(all_docs.clone()))
        .tags(extract_tags(all_docs))
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
