use reqwest::{Client, Response};
use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use tessera::{
    configuration::{get_configuration, DatabaseSettings},
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use uuid::Uuid;

// Ensures that the `TRACING` stack is only initialized once.
static TRACING: Once = Once::new();

/// Initializes telemetry.
///
/// Only execute the following code once.
fn initialize_telemetry() {
    TRACING.call_once(|| {
        let subscriber_name = "test".to_string();
        let filter_name = "info".to_string();

        // Check if the `TEST_LOG` environment variable is set.
        //
        // Print logs if set, otherwise discard logs.
        if std::env::var("TEST_LOG").is_ok() {
            let subscriber = get_subscriber(subscriber_name, filter_name, std::io::stdout);
            init_subscriber(subscriber);
        } else {
            let subscriber = get_subscriber(subscriber_name, filter_name, std::io::sink);
            init_subscriber(subscriber);
        };
    })
}

/// Representation of a test application.
pub struct TestApp {
    pub db_pool: PgPool,
    pub address: String,
}

impl TestApp {
    /// Creates a `GET` request, send it at `/health_check` and then return the response.
    pub async fn get_health_check(&self) -> Response {
        Client::new()
            .get(&format!("{}/health_check", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `POST` request, send it at `/tickets/new` and then return the response.
    pub async fn post_tickets(&self, body: String) -> Response {
        Client::new()
            .post(&format!("{}/tickets/new", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

/// Creates and then run the test application.
pub async fn create_and_run_test_app() -> TestApp {
    initialize_telemetry();

    let configuration = {
        let mut configuration =
            get_configuration().expect("Failed to get the configuration values");
        // Randomize the name of the database.
        configuration.database.database_name = Uuid::new_v4().to_string();
        // Find a random available port by triggering an OS scan using the port 0.
        configuration.application.port = 0;

        configuration
    };

    configure_database(&configuration.database).await;

    let application =
        Application::new(configuration.clone()).expect("Failed to create the application");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        db_pool: configuration.database.get_connection_pool(),
        address: format!("http://127.0.0.1:{}", application_port),
    }
}

/// Returns a pool to a newly created and migrated database.
///
/// # Implementation Notes
///
/// Requires manual clean up.
pub async fn configure_database(database: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&database.without_db())
        .await
        .expect("Failed to connect to the Postgres instance");
    connection
        .execute(&*format!(
            r#"
            CREATE DATABASE "{}";
            "#,
            database.database_name
        ))
        .await
        .expect("Failed to create the database");

    let connection_pool = PgPool::connect_with(database.with_db())
        .await
        .expect("Failed to connect to the database");
    migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
