pub mod users;
pub mod sessions;

use actix_web::{web, HttpResponse};
use serde::Serialize;

/// Runs a blocking Diesel query on actix-web's blocking thread pool and
/// turns the result (or a pool/thread-pool failure) into an HTTP response.
pub async fn respond<T, F>(f: F) -> HttpResponse
where
    F: FnOnce() -> Result<T, diesel::result::Error> + Send + 'static,
    T: Serialize + Send + 'static,
{
    match web::block(f).await {
        Ok(Ok(value)) => HttpResponse::Ok().json(value),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(e.to_string()),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
