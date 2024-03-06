use crate::auth::auth_controller::{login, logout, refresh_token, register, user_info};
use actix_web::web;

pub mod auth_controller;
pub mod auth_service;
pub mod dto;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(refresh_token)
            .service(user_info)
            .service(logout),
    );
}
