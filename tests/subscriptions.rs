use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use voc_sql::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};

struct TestApp {
    address: String,
    pool: PgPool,
}

#[tokio::test]
async fn subscribe_returns_201_on_success() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/json")
        .body(
            r#"{
              "name": "Emil",
              "email": "emilt@randommail.com"
            }"#,
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 201);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "emilt@randommail.com");
    assert_eq!(saved.name, "Emil");
}

#[tokio::test]
async fn subscribe_returns_400_on_missing_email() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/json")
        .body(
            r#"{
              "name": "Emil"
            }"#,
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn subscribe_returns_400_on_missing_name() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/json")
        .body(
            r#"{
              "email": "emil.taciyev@gmail.com"
            }"#,
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 400);
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&configuration.database).await;

    let server = run(listener, pool.clone()).expect("Failed to bind address");
    let address: String = format!("http://127.0.0.1:{}", port);

    let _ = tokio::spawn(server);

    TestApp { address, pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.nameless_connection_string(&config.database_name))
            .await
            .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
