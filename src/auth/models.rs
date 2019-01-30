use diesel::sql_types::Timestamptz;
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub title: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
    pub createdAt: Timestamptz,
    pub updatedAt: Timestamptz,
}
