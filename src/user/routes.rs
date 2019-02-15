use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest,
    State, HttpMessage, error, Json
};
use crate::state::AppState;
use crate::db;

use futures::{future, Future, Stream};

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String,
    password: String,
}


fn create_user((item, state): (Json<MyUser>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
    let copy = item.into_inner();

    state.db
        .send(db::user::CreateUser {
            name: copy.name.clone(),
            password: copy.password.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}