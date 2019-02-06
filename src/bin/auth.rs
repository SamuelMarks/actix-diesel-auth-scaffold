//use actix_diesel_auth_scaffold::{auth, config, user};
//use actix_web::{http::Method, server, App, HttpRequest, Path, Responder};
//use diesel::prelude::*;
//
//use std::env;
//
//fn index(_req: HttpRequest) -> impl Responder {
//    "[auth.rs] `index`"
//}
//
//fn hello(path: Path<String>) -> impl Responder {
//    format!("[auth.rs] `hello` with arg: {}", *path)
//}
//
//fn main() {
//    let port: u16 = std::env::var("PORT")
//        .or(Ok("8000".to_string()) as Result<String, std::env::VarError>)
//        .unwrap()
//        .parse()
//        .unwrap();
//
//    server::new(|| {
//        vec![
//            auth::routes::get_routes(),
//            user::routes::get_routes(),
//            App::new()
//                .resource("/", |r| r.method(Method::GET).with(index))
//                .resource("/hello/{name}", |r| r.method(Method::GET).with(hello)),
//        ]
//    })
//    .bind(format!("127.0.0.1:{}", port))
//    .expect(&*format!("Can not bind to port {}", port))
//    .run();
//}

//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, State,
};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures::Future;

use db::{CreateUser, DbExecutor};

/// State with DbExecutor address
struct AppState {
    db: Addr<DbExecutor>,
}

/// Async request handler
fn index((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("diesel-example");

    // Start 3 db executor actors
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
            .resource("/{name}", |r| r.method(http::Method::GET).with(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
