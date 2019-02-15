use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;

use actix_web::actix::*;

use crate::models;
use crate::schema;
use crate::state::AppState;
use crate::config::SECRET;

use actix_web::error::*;
use actix_web::error;


pub struct CreateUser {
    pub name: String,
    pub password: String,
}


impl Message for CreateUser {
    type Result = Result<models::user::User, Error>;
}


impl Handler<CreateUser> for super::DbExecutor {
    type Result = Result<models::user::User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use self::schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::user::NewUser {
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
            .load::<models::user::User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}
