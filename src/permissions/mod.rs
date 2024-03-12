use actix_web::web;

pub mod permissions_controller;
pub mod permissions_service;
pub mod permissions_repository;
pub mod entities;
pub mod dto;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/permissions")
            .service(permissions_controller::find_all)
            .service(permissions_controller::find_by_id)
            .service(permissions_controller::create)
            .service(permissions_controller::update)
            .service(permissions_controller::delete),
    );
}