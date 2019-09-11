extern crate serde_derive;

use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    HttpRequest, HttpResponse, Path, Responder, Result, http::Method, App, State, Json
};
use diesel::{prelude::*, result::Error as DieselError, sqlite::SqliteConnection};
use serde::Serialize;
use super::super::{Error, Database, schema::users};

pub struct AppState {
    db: Database<SqliteConnection>,
}

fn index(_req: HttpRequest) -> impl Responder {
    "auth_routes Hello from the index page"
}

fn hello(path: Path<String>) -> impl Responder {
    format!("auth_routes Hello {}!", *path)
}

#[inline(always)]
pub fn get_routes() -> App {
    let prefix = format!("/{}", &*module_path!().split("::").nth(1).unwrap());
    println!("[auth_routes.rs] prefix: {}", prefix);
    App::new()
        .prefix(prefix)
        .resource("/", |r| r.method(Method::GET).with(index))
        .resource("/hello/{name}", |r| r.method(Method::GET).with(hello))
}

#[derive(Serialize, Queryable)]
struct User {
    id: i32,
    name: String,
}

pub fn get_all(state: State<AppState>) -> Result<impl Responder> {
    let results = users::table.load::<User>(&state.db)?;

    Ok(Json(results))
}

pub fn get_one(state: State<AppState>, name: Path<String>) -> Result<impl Responder> {
    let result = users::table
        .filter(users::name.eq(name.into_inner()))
        .get_result::<User>(&state.db)
        .map_err(|err| match err {
            Error::Execute(DieselError::NotFound) => ErrorNotFound(err),
            _ => ErrorInternalServerError(err),
        })?;

    Ok(Json(result))
}

#[derive(Insertable)]
#[table_name = "users"]
struct CreateUser {
    name: String,
}

pub fn create(state: State<AppState>, name: Path<String>) -> Result<impl Responder> {
    diesel::insert_into(users::table)
        .values(CreateUser {
            name: name.into_inner()
        })
        .execute(&state.db)?;

    Ok(HttpResponse::Created())
}
