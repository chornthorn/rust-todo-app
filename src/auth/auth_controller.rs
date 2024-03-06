use crate::auth::auth_service::AuthService;
use crate::auth::dto::{LoginDto, RegisterDto};
use crate::config::AppConfig;
use crate::shared::jwt_middleware::JwtMiddleware;
use crate::shared::jwt_refresh_token::JwtRefreshToken;
use crate::shared::response::JsonResponder;
use crate::shared::token_claim::TokenClaims;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use validator::Validate;

#[post("login")]
async fn login(body: web::Json<LoginDto>, data: web::Data<AppConfig>) -> impl Responder {
    match body.validate() {
        Ok(_) => {
            AuthService::new(data.pool.clone())
                .login(body.into_inner())
                .await
        }
        Err(err) => JsonResponder::validation_error(err),
    }
}

#[post("register")]
async fn register(dto: web::Json<RegisterDto>, data: web::Data<AppConfig>) -> impl Responder {
    match dto.validate() {
        Ok(_) => {
            AuthService::new(data.pool.clone())
                .register(dto.into_inner())
                .await
        }
        Err(err) => JsonResponder::validation_error(err),
    }
}

#[post("/logout")]
async fn logout() -> impl Responder {
    JsonResponder::ok("Logout successful", None)
}

#[post("/token/refresh")]
async fn refresh_token(data: web::Data<AppConfig>, token: JwtRefreshToken) -> impl Responder {
    AuthService::new(data.pool.clone())
        .refresh_token(token.user_id)
        .await
}

#[get("/user/info")]
async fn user_info(data: web::Data<AppConfig>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<TokenClaims>().unwrap().sub;
    AuthService::new(data.pool.clone()).user_info(user_id).await
}
