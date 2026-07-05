use actix_web::{HttpRequest, HttpResponse, web};
use crate::models::user::{UserList, UsersTemplate, NewUser, User, Webhook};
use crate::handlers::respond;
use askama::Template;
use std::sync::Arc;
use crate::db_connection::DbPool;

pub async fn index(pool: web::Data<Arc<DbPool>>, _req: HttpRequest) -> HttpResponse {
    let pool = pool.get_ref().clone();
    match web::block(move || UserList::list(pool)).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn latest(pool: web::Data<Arc<DbPool>>, _req: HttpRequest) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let rendered = web::block(move || UsersTemplate::latest(pool).render()).await;

    match rendered {
        Ok(Ok(body)) => HttpResponse::Ok().body(body),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(e.to_string()),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn create(pool: web::Data<Arc<DbPool>>, new_user: web::Json<NewUser>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let new_user = new_user.into_inner();
    respond(move || new_user.create(pool)).await
}

pub async fn show(pool: web::Data<Arc<DbPool>>, Id: web::Path<i32>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let id = *Id;
    respond(move || User::find(&id, pool)).await
}

pub async fn destroy(pool: web::Data<Arc<DbPool>>, Id: web::Path<i32>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let id = *Id;
    respond(move || User::destroy(&id, pool)).await
}

pub async fn update(pool: web::Data<Arc<DbPool>>, Id: web::Path<i32>, user: web::Json<User>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let id = *Id;
    let user = user.into_inner();
    respond(move || User::update(&id, &user, pool)).await
}

pub async fn upsert(pool: web::Data<Arc<DbPool>>, webhook: web::Json<Webhook>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let webhook = webhook.into_inner();
    respond(move || User::upsert(&webhook, pool)).await
}
