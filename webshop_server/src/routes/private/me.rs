use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use log::error;
use utoipa::{OpenApi, ToSchema};

use crate::{
    utils::auth::{validate_user, AuthError},
    SharedData,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(me);
}

#[derive(OpenApi)]
#[openapi(
    paths(
        me
    ),
    components(
        schemas(MeUser)
    ),
    tags(
        (name = "me", description = "User related routes")
    ),
)]
pub struct UserApiDoc;

#[derive(serde::Serialize, ToSchema)]
struct MeUser {
    user_id: i32,
    email: String,
    role: String,
    company_id: i32,
}

impl From<crate::data_access::user::User> for MeUser {
    fn from(user: crate::data_access::user::User) -> Self {
        Self {
            user_id: user.user_id,
            email: user.email,
            role: user.role.to_string(),
            company_id: user.company_id,
        }
    }
}

#[utoipa::path(
        context_path = "/api/priv",
    tag = "me",
    responses(
    (status = 200, description = "JSON containing the user information"),
    (status = 401, description = "Unauthorized"),
    (status = 500, description = "Internal Server Error"
    )
    )
)]
#[get("/me")]
async fn me(shared_data: web::Data<SharedData>, req: HttpRequest) -> impl Responder {
    let pool = &shared_data.db_pool;
    let user = validate_user(req, pool).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(MeUser::from(user)),
        Err(e) => match e {
            AuthError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AuthError::SqlxError(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}
