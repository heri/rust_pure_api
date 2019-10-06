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

pub fn update_or_create(user: web::Json<User>) -> Result<HttpResponse, HttpResponse> {
    User::find(user.Id)
        .map(|user| 
            User::update(&Id, &new_user)
                .map(|_| HttpResponse::Ok().json(()))
                .map_err(|e| {
                    HttpResponse::InternalServerError().json(e.to_string())
                })
        )
        .map_err(|e| {
            user.create()
                .map(|user| HttpResponse::Ok().json(user))
                .map_err(|e| {
                    HttpResponse::InternalServerError().json(e.to_string())
                })
        })
}

use crate::models::user::User;

pub fn show(Id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    User::find(&Id)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn destroy(Id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    User::destroy(&Id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn update(Id: web::Path<i32>, new_user: web::Json<NewUser>) -> Result<HttpResponse, HttpResponse> {
    User::update(&Id, &new_user)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}