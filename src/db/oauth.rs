use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;

use actix_web::actix::*;

use actix_web::error::*;
use actix_web::error;

use crate::models;
use crate::state::AppState;
use crate::config::SECRET;
use crate::oauth;

pub struct GetToken {
    pub grant_type: String,
    pub username: String,
    pub password: String,

}

impl Message for GetToken {
    type Result = Result<oauth::Token, Error>;
}

pub struct Authorise {
    pub username: String,
    pub password: String,
}

impl Message for Authorise {
    type Result = Result<oauth::Token, Error>;
}


impl Handler<GetToken> for super::DbExecutor {
    type Result = Result<oauth::Token, Error>;

    fn handle(&mut self, msg: GetToken, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let mut items = users
            .filter(name.eq(&msg.username))
            .load::<models::user::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        //TODO: fix unwrap
        let user = items.pop().unwrap();


        let claims = oauth::Claims{
            username: msg.username.clone(),
        };

        let access_token = oauth::generate_token(claims, SECRET.to_string());

        Ok(oauth::Token {
            access_token: access_token.to_string(),
            token_type: oauth::TokenType::Bearer,
            expires_in: 1500,

        })
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