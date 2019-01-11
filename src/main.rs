use actix_diesel_auth_scaffold::auth;
use actix_diesel_auth_scaffold::user;
use actix_web::{http::Method, server, App, HttpRequest, Path, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page"
}

fn hello(path: Path<String>) -> impl Responder {
    format!("Hello {}!", *path)
}

fn main() {
    let port: u16 = std::env::var("PORT")
        .or(Ok("8000".to_string()) as Result<String, std::env::VarError>)
        .unwrap()
        .parse()
        .unwrap();

    server::new(|| {
        vec![
            App::new()
                .resource("/", |r| r.method(Method::GET).with(index))
                .resource("/hello/{name}", |r| r.method(Method::GET).with(hello)),
            auth::routes::get_routes() ,
            user::routes::get_routes(),
        ]
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect(&*format!("Can not bind to port {}", port))
    .run();
}
