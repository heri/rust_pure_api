use actix_web::{HttpRequest, HttpResponse };

use crate::models::user::UserList;

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(UserList::list())
}

use crate::models::user::NewUser;
use actix_web::web;

pub fn create(new_user: web::Json<NewUser>) -> Result<HttpResponse, HttpResponse> {
    new_user
        .create()
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

use crate::models::user::User;

pub fn show(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    User::find(&id)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn destroy(id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    User::destroy(&id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn update(id: web::Path<i32>, user: web::Json<User>) -> Result<HttpResponse, HttpResponse> {
    User::update(&id, &user)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn upsert(id: web::Path<i32>, user: web::Json<User>) -> Result<HttpResponse, HttpResponse> {
    User::upsert(&id, &user)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}