use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;


use actix_web::actix::*;

use crate::models;
use crate::schema;
use crate::oauth;
use crate::state::AppState;
use crate::config::SECRET;

use actix_web::error::*;
use actix_web::error;

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

pub struct CreateUser {
    pub name: String,
    pub password: String,
}


impl Message for CreateUser {
    type Result = Result<models::User, Error>;
}

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

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::NewUser {
            id: &uuid,
            name: &msg.name,
            password: &msg.password,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<models::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}


impl Handler<GetToken> for DbExecutor {
    type Result = Result<oauth::Token, Error>;

    fn handle(&mut self, msg: GetToken, ctx: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let mut items = users
            .filter(name.eq(&msg.username))
            .load::<models::User>(conn)
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