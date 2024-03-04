use actix_web::web;
use crate::todos::todos_controller::{get_all_todos, create_todo, update_todo, delete_todo};

pub mod todos_controller;
pub mod todos_service;
pub mod todos_repository;
pub mod dto;
pub mod entities;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(get_all_todos)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    );
}