use crate::db::DbExecutor;
use actix::Addr;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

