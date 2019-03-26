use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;

use actix_web::actix::*;

use actix_web::error;
use actix_web::error::*;

use super::Claims;
use crate::user::model::{NewUser, User};
use crate::DbExecutor;
use crate::SECRET;

use crate::oauth2::AccessToken;

pub struct GetToken {
    pub grant_type: String,
    pub username: String,
    pub password: String,
}

impl Message for GetToken {
    type Result = Result<AccessToken, Error>;
}

pub struct Authorise {
    pub username: String,
    pub password: String,
}

impl Message for Authorise {
    type Result = Result<AccessToken, Error>;
}

impl Handler<GetToken> for DbExecutor {
    type Result = Result<AccessToken, Error>;

    fn handle(&mut self, msg: GetToken, ctx: &mut Self::Context) -> Self::Result {
        use crate::user::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let mut items = users
            .filter(name.eq(&msg.username))
            .load::<User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        //TODO: fix unwrap
        let user = items.pop().unwrap();

        let claims = Claims {
            username: msg.username.clone(),
        };

        let access_token = super::generate_token(claims, SECRET.to_string());

        Ok(AccessToken(access_token.to_string()))
    }
}
//
//impl Handler<VerifyToken> for DbExecutor {
//    type Result = Result<oauth::Token, Error>;
//
//    fn handle(&mut self, msg: GetToken, _: &mut Self::Context) -> Self::Result {
//        use self::schema::users::dsl::*;
//
//
//    }
//}

//
//fn authorise((item, state): (Json<oauth::Token>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    let copy = item.into_inner();
//
//    oauth::is_token_valid(copy, state.secret.clone())
//
//
//}
