use actix_web::{  HttpRequest, HttpResponse };

use crate::models::user::UserList;
use crate::models::user::UsersTemplate;
use askama::Template;

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(UserList::list())
}

// Faster Bare Metal Alternative https://yarte.netlify.com/with_actix_web.html
pub fn latest(_req: HttpRequest) -> HttpResponse {
    let tmpl = UsersTemplate::latest();
    let res = tmpl.render().unwrap();

    HttpResponse::Ok()
        .body(res)
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
use crate::models::user::Webhook;

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

pub fn update(Id: web::Path<i32>, user: web::Json<User>) -> Result<HttpResponse, HttpResponse> {
    User::update(&Id, &user)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub fn upsert(webhook: web::Json<Webhook>) -> Result<HttpResponse, HttpResponse> {
    User::upsert(&webhook)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}