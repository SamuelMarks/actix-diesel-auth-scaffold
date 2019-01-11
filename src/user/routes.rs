use actix_web::{http::Method, App, HttpRequest, Path, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "[user/routes.rs] Hello from the index page"
}

fn hello(path: Path<String>) -> impl Responder {
    format!("[user/routes.rs] `hello` with arg: {}", *path)
}

#[inline(always)]
pub fn get_routes() -> App {
    App::new()
        .prefix(format!("/{}", &*module_path!().split("::").nth(1).unwrap()))
        .resource("/", |r| r.method(Method::GET).with(index))
        .resource("/hello/{name}", |r| r.method(Method::GET).with(hello))
}
