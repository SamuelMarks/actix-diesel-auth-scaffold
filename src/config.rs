use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2_postgres::PostgresConnectionManager;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_url = "127.0.0.1:5432";
    PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
}
