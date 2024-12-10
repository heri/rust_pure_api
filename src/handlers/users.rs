use actix_web::{HttpRequest, HttpResponse, web};
use crate::models::user::{UserList, UsersTemplate, NewUser, User, Webhook};
use askama::Template;
use std::sync::Arc;
use crate::db_connection::DbPool;

pub fn index(pool: web::Data<Arc<DbPool>>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(UserList::list(pool.get_ref().clone()))
}

pub fn latest(pool: web::Data<Arc<DbPool>>, _req: HttpRequest) -> HttpResponse {
    let tmpl = UsersTemplate::latest(pool.get_ref().clone());
    let res = tmpl.render().unwrap();

    HttpResponse::Ok().body(res)
}

pub fn create(pool: web::Data<Arc<DbPool>>, new_user: web::Json<NewUser>) -> Result<HttpResponse, HttpResponse> {
    new_user
        .create(pool.get_ref().clone())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn show(pool: web::Data<Arc<DbPool>>, Id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    User::find(&Id, pool.get_ref().clone())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn destroy(pool: web::Data<Arc<DbPool>>, Id: web::Path<i32>) -> Result<HttpResponse, HttpResponse> {
    User::destroy(&Id, pool.get_ref().clone())
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn update(pool: web::Data<Arc<DbPool>>, Id: web::Path<i32>, user: web::Json<User>) -> Result<HttpResponse, HttpResponse> {
    User::update(&Id, &user, pool.get_ref().clone())
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn upsert(pool: web::Data<Arc<DbPool>>, webhook: web::Json<Webhook>) -> Result<HttpResponse, HttpResponse> {
    User::upsert(&webhook, pool.get_ref().clone())
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}