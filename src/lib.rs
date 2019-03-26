pub mod auth;
pub mod oauth2;
pub mod user;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use actix_web::actix::*;
use actix_web::*;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use uuid;

pub const SECRET: &str = "secret";

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
