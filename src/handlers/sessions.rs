use actix_web::{  HttpRequest, HttpResponse };

use crate::models::session::SessionList;

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(SessionList::list())
}

pub fn for_user(player_number: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(SessionList::for_user(&player_number))
}

use actix_web::web;

use crate::models::session::Session;

pub fn upsert(session: web::Json<Session>) -> Result<HttpResponse, HttpResponse> {
    Session::upsert(&session)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}