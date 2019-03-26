use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest,
    State, HttpMessage, error, Json
};
use crate::AppState;


use futures::{future, Future, Stream};

#[derive(Debug, Serialize, Deserialize, Message)]
pub struct GetTokenReq {
    username: String,
    grant_type: String,
    password: String,
}

pub fn get_token((item, state): (Json<GetTokenReq>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {

    let copy = item.into_inner();

    state
        .db
        .send(super::db::GetToken {
            grant_type: "password".to_string(),
            username: copy.username.clone(),
            password: copy.password.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })


}