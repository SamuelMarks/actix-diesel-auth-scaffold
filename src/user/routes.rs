use actix_web::{http::Method, App, HttpRequest, Path, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "[user_routes.rs] Hello from the index page"
}

fn hello(path: Path<String>) -> impl Responder {
    format!("[user_routes.rs] Hello {}!", *path)
}

#[inline(always)]
pub fn get_routes() -> App {
    let prefix = format!("/{}", &*module_path!().split("::").nth(1).unwrap());
    println!("[user_routes.rs] prefix: {}", prefix);
    App::new()
        .prefix(prefix)
        .resource("/", |r| r.method(Method::GET).with(index))
        .resource("/hello/{name}", |r| r.method(Method::GET).with(hello))
}
