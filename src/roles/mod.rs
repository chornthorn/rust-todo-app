use actix_web::web;

pub mod roles_controller;
pub mod roles_service;
pub mod roles_repository;
pub mod dto;
pub mod entities;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
    );
}
