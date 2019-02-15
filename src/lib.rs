pub mod user;
pub mod auth;
pub mod oauth2;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use crate::db::DbExecutor;
use actix::Addr;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}



use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;

use actix_web::actix::*;

use actix_web::error::*;
use actix_web::error;

pub const SECRET: &str = "secret";

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}



