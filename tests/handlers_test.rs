// tests/integration_tests.rs

use actix_web::{http::StatusCode, test, web, App};
use rs_backend::handlers::{create_user, delete_user, get_user, update_user};
use rs_backend::models::{NewUser, UpdateUser};
use sqlx::SqlitePool;
use std::env;

async fn setup_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            email TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

#[actix_rt::test]
async fn test_create_user() {
    let pool = setup_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::post().to(create_user)),
    )
    .await;

    let new_user = NewUser {
        nombre: "Juan Pérez".into(),
        email: "juan.perez@example.com".into(),
    };

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[actix_rt::test]
async fn test_get_user() {
    let pool = setup_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users/{id}", web::get().to(get_user)),
    )
    .await;

    // Insert a user into the database
    let new_user = NewUser {
        nombre: "Juan Pérez".into(),
        email: "juan.perez@example.com".into(),
    };
    sqlx::query!(
        "INSERT INTO users (nombre, email) VALUES (?, ?)",
        new_user.nombre,
        new_user.email
    )
    .execute(&pool)
    .await
    .unwrap();

    let req = test::TestRequest::get().uri("/users/1").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_update_user() {
    let pool = setup_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users/{id}", web::put().to(update_user)),
    )
    .await;

    // Insert a user into the database
    let new_user = NewUser {
        nombre: "Juan Pérez".into(),
        email: "juan.perez@example.com".into(),
    };
    sqlx::query!(
        "INSERT INTO users (nombre, email) VALUES (?, ?)",
        new_user.nombre,
        new_user.email
    )
    .execute(&pool)
    .await
    .unwrap();

    let updated_user = UpdateUser {
        nombre: "Juan Actualizado".into(),
        email: "juan.actualizado@example.com".into(),
    };

    let req = test::TestRequest::put()
        .uri("/users/1")
        .set_json(&updated_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_delete_user() {
    let pool = setup_db().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users/{id}", web::delete().to(delete_user)),
    )
    .await;

    // Insert a user into the database
    let new_user = NewUser {
        nombre: "Juan Pérez".into(),
        email: "juan.perez@example.com".into(),
    };
    sqlx::query!(
        "INSERT INTO users (nombre, email) VALUES (?, ?)",
        new_user.nombre,
        new_user.email
    )
    .execute(&pool)
    .await
    .unwrap();

    let req = test::TestRequest::delete().uri("/users/1").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
