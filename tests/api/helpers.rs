use sqlx::{migrate, Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use tessera::{
    configuration::{get_configuration, DatabaseSettings},
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use uuid::Uuid;

// Ensure that the `TRACING` stack is only initialized once.
static TRACING: Once = Once::new();

/// Initialize telemetry.
fn initialize_telemetry() {
    TRACING.call_once(|| {
        // Set the name of the filter and the subscriber.
        let filter_name = "info".to_string();
        let subscriber_name = "test".to_string();

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

pub struct TestApp {
    pub db_pool: PgPool,
    pub address: String,
}

impl TestApp {
    /// Create a `GET` request, send it at `/health_check` and then return the response.
    pub async fn get_health_check(&self) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/health_check", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Create a `POST` request, send it at `/tickets` and then return the response.
    pub async fn post_tickets(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/tickets", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

/// Build and then run the test application.
pub async fn spawn_test_app() -> TestApp {
    // Initialize telemetry.
    //
    // Only execute the following code once.
    initialize_telemetry();

    // Get the configuration values.
    let configuration = {
        let mut configuration =
            get_configuration().expect("Failed to get the configuration values");
        // Randomize database name.
        configuration.database.database_name = Uuid::new_v4().to_string();
        // Bind the socket address by triggering an OS scan using the port 0,
        // to find a random available port.
        configuration.application.port = 0;

        configuration
    };

    // Create a migrated Postgres database.
    configure_database(&configuration.database).await;

    // Build the application.
    let application =
        Application::build(configuration.clone()).expect("Failed to build the application");

    // Get the port.
    let application_port = application.port();

    // Run the application.
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        db_pool: configuration.database.get_connection_pool(),
        address: format!("http://127.0.0.1:{}", application_port),
    }
}

/// Return a pool to a newly created and migrated Postgres database.
///
/// # Implementation Notes
///
/// Require manual clean up.
pub async fn configure_database(database: &DatabaseSettings) -> PgPool {
    // Connect to the Postgres instance.
    let mut connection = PgConnection::connect_with(&database.without_db())
        .await
        .expect("Failed to connect to the Postgres instance");

    // Create the Postgres database.
    connection
        .execute(&*format!(
            r#"
            CREATE DATABASE "{}";
            "#,
            database.database_name
        ))
        .await
        .expect("Failed to create the Postgres database");

    // Connect to the Postgres database.
    let connection_pool = PgPool::connect_with(database.with_db())
        .await
        .expect("Failed to connect to the Postgres database");

    // Run migrations.
    migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the Postgres database");

    connection_pool
}
