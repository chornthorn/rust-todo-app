use actix_web::web;
use crate::roles::roles_controller::{create, delete, find_all, find_by_id, update};

pub mod roles_controller;
pub mod roles_service;
pub mod roles_repository;
pub mod dto;
pub mod entities;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .service(find_all)
            .service(find_by_id)
            .service(create)
            .service(update)
            .service(delete),
    );
}
