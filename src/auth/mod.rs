use actix_web::web;
use crate::auth::auth_controller::{login, logout, refresh_token, register, user_info};

pub mod dto;
pub mod auth_controller;
pub mod auth_service;
pub mod auth_repository;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
            .service(refresh_token)
            .service(user_info)
            .service(logout)
    );
}