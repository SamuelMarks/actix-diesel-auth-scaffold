#[macro_use]
extern crate lazy_static;

use actix_diesel_auth_scaffold::{auth, config, user};
use actix_web::{http::Method, server, App, HttpRequest, Path, Responder};

lazy_static! {
    pub static ref CONNECTION: diesel::pg::PgConnection = config::establish_connection();
}

// TODO: Replace with https://docs.diesel.rs/r2d2/index.html

fn index(_req: HttpRequest) -> impl Responder {
    "[main.rs] `index`"
}

fn hello(path: Path<String>) -> impl Responder {
    format!("[main.rs] `hello` with arg: {}", *path)
}

fn main() {
    let port: u16 = std::env::var("PORT")
        .or(Ok("8000".to_string()) as Result<String, std::env::VarError>)
        .unwrap()
        .parse()
        .unwrap();

    server::new(|| {
        vec![
            auth::routes::get_routes(),
            user::routes::get_routes(),
            App::new()
                .resource("/", |r| r.method(Method::GET).with(index))
                .resource("/hello/{name}", |r| r.method(Method::GET).with(hello)),
        ]
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect(&*format!("Can not bind to port {}", port))
    .run();
}
