pub mod db;
pub mod model;
pub mod routes;
pub mod schema;

#[cfg(test)]
mod tests {

    use crate::user::db::CreateUser;
    use actix_web::test::TestServer;
    use actix_web::{http, App, HttpMessage};
    use futures::future::Future;

    use actix::prelude::*;

    use diesel::prelude::*;

    use crate::DbExecutor;

    use diesel::r2d2::{ConnectionManager, Pool};

    fn init_test_app() -> App<crate::AppState> {
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

        App::with_state(crate::AppState { db: addr.clone() }).resource("/user", |r| {
            r.method(http::Method::POST)
                .with_async(super::routes::create_user)
        })
    }

    #[test]
    fn test_create_user() {
        let mut server = TestServer::with_factory(init_test_app);

        let request = server
            .client(http::Method::POST, "/user")
            .json(super::routes::MyUser {
                name: "bob".to_string(),
                password: "dole".to_string(),
            })
            .unwrap();

        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
    }

}
