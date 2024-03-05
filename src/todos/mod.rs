use crate::todos::todos_controller::{create_todo, delete_todo, get_all_todos, update_todo};
use actix_web::web;

pub mod dto;
pub mod entities;
pub mod todos_controller;
pub mod todos_repository;
pub mod todos_service;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(get_all_todos)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo),
    );
}
