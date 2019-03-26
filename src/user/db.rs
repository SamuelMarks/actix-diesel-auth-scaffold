use actix_web::*;
use uuid;

use actix_web::actix::*;

use actix_web::error;
use actix_web::error::*;

use diesel::*;

use crate::DbExecutor;

use super::model::{NewUser, User};
use super::schema;
use super::schema::users::dsl::*;

pub struct CreateUser {
    pub name: String,
    pub password: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = NewUser {
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
            .load::<User>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
    }
}
