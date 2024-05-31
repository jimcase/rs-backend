use super::models::{NewUser, UpdateUser, User, Users};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sea_query::{Expr, Query, SqliteQueryBuilder, Value};
use serde_json::json;
use sqlx::{sqlite::SqliteArguments, SqlitePool};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Bienvenido al servidor API!")
}

pub async fn create_user(
    new_user: web::Json<NewUser>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let (sql, values) = Query::insert()
        .into_table(Users::Table)
        .columns(vec![Users::Nombre, Users::Email])
        .values_panic(vec![
            new_user.nombre.clone().into(),
            new_user.email.clone().into(),
        ])
        .build(SqliteQueryBuilder);

    let mut query = sqlx::query(&sql);
    for value in values.0 {
        // Acceder al Vec<Value> interno
        query = bind_query_value(query, value);
    }

    let result = query.execute(db_pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"message": "Usuario creado con éxito"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn get_user(req: HttpRequest, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let user_id = req.match_info().get("id").unwrap_or("0").parse::<i64>();

    match user_id {
        Ok(id) => {
            let (sql, values) = Query::select()
                .columns(vec![Users::Id, Users::Nombre, Users::Email])
                .from(Users::Table)
                .and_where(Expr::col(Users::Id).eq(id))
                .build(SqliteQueryBuilder);

            let mut query = sqlx::query_as::<_, User>(&sql);
            for value in values.0 {
                // Acceder al Vec<Value> interno
                query = bind_query_as_value(query, value);
            }

            let result = query.fetch_optional(db_pool.get_ref()).await;

            match result {
                Ok(Some(user)) => HttpResponse::Ok().json(user),
                Ok(None) => HttpResponse::NotFound().json("Usuario no encontrado"),
                Err(_) => {
                    HttpResponse::InternalServerError().json("Error al consultar la base de datos")
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().json("ID inválido"),
    }
}

pub async fn update_user(
    id: web::Path<i64>,
    updated_user: web::Json<UpdateUser>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let (sql, values) = Query::update()
        .table(Users::Table)
        .values(vec![
            (Users::Nombre, updated_user.nombre.clone().into()),
            (Users::Email, updated_user.email.clone().into()),
        ])
        .and_where(Expr::col(Users::Id).eq(*id))
        .build(SqliteQueryBuilder);

    let mut query = sqlx::query(&sql);
    for value in values.0 {
        // Acceder al Vec<Value> interno
        query = bind_query_value(query, value);
    }

    let result = query.execute(db_pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Usuario actualizado con éxito"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn delete_user(id: web::Path<i64>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let (sql, values) = Query::delete()
        .from_table(Users::Table)
        .and_where(Expr::col(Users::Id).eq(*id))
        .build(SqliteQueryBuilder);

    let mut query = sqlx::query(&sql);
    for value in values.0 {
        // Acceder al Vec<Value> interno
        query = bind_query_value(query, value);
    }

    let result = query.execute(db_pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Usuario eliminado"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

fn bind_query_value<'q>(
    query: sqlx::query::Query<'q, sqlx::Sqlite, SqliteArguments<'q>>,
    value: Value,
) -> sqlx::query::Query<'q, sqlx::Sqlite, SqliteArguments<'q>> {
    match value {
        Value::Bool(boolean) => query.bind(boolean),
        Value::TinyInt(int) => query.bind(int),
        Value::SmallInt(int) => query.bind(int),
        Value::Int(int) => query.bind(int),
        Value::BigInt(bigint) => query.bind(bigint),
        Value::Float(float) => query.bind(float),
        Value::Double(double) => query.bind(double),
        Value::String(Some(text)) => query.bind(*text), // Desreferenciar Box<String>
        Value::Bytes(Some(bytes)) => query.bind(*bytes), // Desreferenciar Box<Vec<u8>>
        Value::String(None) => query.bind::<Option<String>>(None),
        Value::Bytes(None) => query.bind::<Option<Vec<u8>>>(None),
        _ => query,
    }
}

fn bind_query_as_value<'q>(
    query: sqlx::query::QueryAs<'q, sqlx::Sqlite, User, SqliteArguments<'q>>,
    value: Value,
) -> sqlx::query::QueryAs<'q, sqlx::Sqlite, User, SqliteArguments<'q>> {
    match value {
        Value::Bool(boolean) => query.bind(boolean),
        Value::TinyInt(int) => query.bind(int),
        Value::SmallInt(int) => query.bind(int),
        Value::Int(int) => query.bind(int),
        Value::BigInt(bigint) => query.bind(bigint),
        Value::Float(float) => query.bind(float),
        Value::Double(double) => query.bind(double),
        Value::String(Some(text)) => query.bind(*text), // Desreferenciar Box<String>
        Value::Bytes(Some(bytes)) => query.bind(*bytes), // Desreferenciar Box<Vec<u8>>
        Value::String(None) => query.bind::<Option<String>>(None),
        Value::Bytes(None) => query.bind::<Option<Vec<u8>>>(None),
        _ => query,
    }
}
