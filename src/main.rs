pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate askama;
#[macro_use] 
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use crate::db_connection::establish_connection_pool;

fn main() {
    let sys = actix::System::new("rust_api");
    let pool = establish_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/v1/users")
                    .route(web::get().to(handlers::users::index))
                    .route(web::post().to(handlers::users::create))
            )
            .service(
                web::resource("/v1/users/{Id}")
                    .route(web::get().to(handlers::users::show))
                    .route(web::delete().to(handlers::users::destroy))
                    .route(web::patch().to(handlers::users::update))
            )
            .service(
                web::resource("/v1/sessions")
                    .route(web::get().to(handlers::sessions::index))
            )
            .service(
                web::resource("/v1/sessions/user/{playerNumber}")
                    .route(web::get().to(handlers::sessions::for_user))
            )
            .service(
                web::resource("/v1/webhook/user")
                    .route(web::post().to(handlers::users::upsert))
            )
            .service(
                web::resource("/v1/webhook/session")
                    .route(web::post().to(handlers::sessions::upsert))
            )
            .service(
                web::resource("/")
                    .route(web::get().to(handlers::users::latest))
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}