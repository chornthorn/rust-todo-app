use crate::users::users_controller::{
    create_user, delete_user, get_user_by_id, index, update_user,
};
use actix_web::web;

pub mod dto;
pub mod entities;
pub mod users_controller;
pub mod users_repository;
pub mod users_service;

// configure the server scope
pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(index)
            .service(create_user)
            .service(get_user_by_id)
            .service(update_user)
            .service(delete_user),
    );
}
