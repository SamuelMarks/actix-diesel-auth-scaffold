use actix::prelude::*;
use actix_web::{
    error, http, middleware, server, App, AsyncResponder, Error, FutureResponse, HttpMessage,
    HttpRequest, HttpResponse, Json, Path, State,
};
use bytes::BytesMut;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures::{future, Future, Stream};

mod db;
mod models;
mod oauth;
mod schema;

use db::{Authorise, CreateUser, DbExecutor, GetToken};

struct AppState {
    db: Addr<DbExecutor>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String,
    password: String,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

fn create_user(
    (item, state): (Json<MyUser>, State<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let copy = item.into_inner();

    state
        .db
        .send(CreateUser {
            name: copy.name.clone(),
            password: copy.password.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

fn get_token(
    (item, state): (Json<oauth::Credentials>, State<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let copy = item.into_inner();

    state
        .db
        .send(GetToken {
            username: copy.username.clone(),
            password: copy.password.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

fn authorise(
    (item, state): (Json<oauth::Token>, State<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let copy = item.into_inner();

    state
        .db
        .send(Authorise {
            username: copy.username.clone(),
            password: copy.password.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("diesel-r2d2-oauth");

    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState { db: addr.clone() })
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/user", |r| {
                r.method(http::Method::POST)
                    .with_async_config(create_user, |(json_cfg,)| {
                        json_cfg.0.limit(4096); // <- limit size of the payload
                    })
            })
            .resource("/token", |r| {
                r.method(http::Method::GET)
                    .with_async_config(get_token, |(json_cfg,)| {
                        json_cfg.0.limit(4096); // <- limit size of the payload
                    })
            })
            .resource("/authorise", |r| {
                r.method(http::Method::POST)
                    .with_async_config(authorise, |(json_cfg,)| {
                        json_cfg.0.limit(4096); // <- limit size of the payload
                    })
            })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
