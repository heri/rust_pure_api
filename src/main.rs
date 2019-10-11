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
#[macro_use] 
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate futures;
use actix_web::{App, HttpServer, web};

fn main() {
    let sys = actix::System::new("rust_pure_api");

    HttpServer::new(
    || App::new()
        .service(
            web::resource("/users")
                .route(web::get().to_async(handlers::users::index))
                .route(web::post().to_async(handlers::users::create))
        )
        .service(
            web::resource("/users/{id}")
                .route(web::get().to_async(handlers::users::show))
                .route(web::delete().to_async(handlers::users::destroy))
                .route(web::patch().to_async(handlers::users::update))
        )
        .service(
            web::resource("/webhook")
                .route(web::post().to_async(handlers::users::upsert))
        )
    )
    .bind("127.0.0.1:8088").unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}