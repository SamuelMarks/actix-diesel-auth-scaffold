extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate bytes;

extern crate actix_diesel_auth_scaffold;



use bytes::BytesMut;
use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest,
    State, HttpMessage, error, Json
};


use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures::{future, Future, Stream};

use actix_diesel_auth_scaffold::oauth2::*;
use actix_diesel_auth_scaffold::user::routes::create_user;
use actix_diesel_auth_scaffold::DbExecutor;
use actix_diesel_auth_scaffold::auth::db::*;
use actix_diesel_auth_scaffold::auth::routes::get_token;
use actix_diesel_auth_scaffold::*;



const MAX_SIZE: usize = 262_144; // max payload size is 256k


fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("diesel-r2d2-oauth");

    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));


    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/user", |r| {
                r.method(http::Method::POST).with_async(create_user)
            })
            .resource("/token", |r| {
                r.method(http::Method::POST).with_async(get_token)

            })
//            .resource("/validate", |r| {
//                r.method(http::Method::POST)
//                    .with_async_config(authorise,|(json_cfg )| {
//                        json_cfg.0.limit(4096);
//                    })
//            })

    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
