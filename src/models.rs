use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Nombre,
    Email,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub nombre: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub nombre: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub nombre: String,
    pub email: String,
}
