#![feature(custom_attribute)]

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub password: &'a str,
}
