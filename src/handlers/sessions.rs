use actix_web::{HttpRequest, HttpResponse, web};
use std::sync::Arc;
use crate::db_connection::DbPool;
use crate::handlers::respond;

use crate::models::session::SessionList;
use crate::models::session::SessionWebhook;

pub async fn index(pool: web::Data<Arc<DbPool>>, _req: HttpRequest) -> HttpResponse {
    let pool = pool.get_ref().clone();
    respond(move || SessionList::list(pool)).await
}

pub async fn for_user(pool: web::Data<Arc<DbPool>>, player_number: web::Path<String>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let player_number = player_number.into_inner();
    respond(move || SessionList::for_user(&player_number, pool)).await
}

pub async fn upsert(pool: web::Data<Arc<DbPool>>, session: web::Json<SessionWebhook>) -> HttpResponse {
    let pool = pool.get_ref().clone();
    let session = session.into_inner();
    respond(move || SessionWebhook::upsert(&session, pool)).await
}
