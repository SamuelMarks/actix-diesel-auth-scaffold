use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest,
    State, HttpMessage, error, Json
};


use crate::AppState;

use futures::{future, Future, Stream};

use super::db::CreateUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyUser {
    pub name: String,
    pub password: String,
}

pub fn create_user((item, state): (Json<MyUser>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
    let copy = item.into_inner();

    state.db
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
