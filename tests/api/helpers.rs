use argon2::{
    password_hash::SaltString,
    {Algorithm, Argon2, Params, PasswordHasher, Version},
};
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
    pub api_client: Client,
    pub test_user: TestUser,
}

/// Representation of a test user.
pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    /// Returns a test user.
    pub fn new() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    /// Stores the test user.
    async fn store(&self, pool: &PgPool) {
        // Create random salt.
        let salt = SaltString::generate(&mut rand::thread_rng());

        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)",
            self.user_id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to save the test user details in the database");
    }

    /// Logs in the test user.
    pub async fn login(&self, app: &TestApp) {
        app.post_login(&serde_json::json!({
            "username": &self.username,
            "password": &self.password
        }))
        .await;
    }

    /// Logs out the test user.
    pub async fn logout(&self, app: &TestApp) {
        app.post_logout().await;
    }
}

impl TestApp {
    /// Creates a `GET` request, send it at `/health_check` and then return the response.
    pub async fn get_health_check(&self) -> Response {
        self.api_client
            .get(&format!("{}/health_check", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `GET` request, send it at `/register` and then return the response.
    pub async fn get_register(&self) -> Response {
        self.api_client
            .get(&format!("{}/register", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Returns the register full response text.
    pub async fn get_register_html(&self) -> String {
        self.get_register().await.text().await.unwrap()
    }

    /// Creates a `POST` request, send it at `/register` and then return the response.
    pub async fn post_register<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/register", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `GET` request, send it at `/login` and then return the response.
    pub async fn get_login(&self) -> Response {
        self.api_client
            .get(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Returns the login full response text.
    pub async fn get_login_html(&self) -> String {
        self.get_login().await.text().await.unwrap()
    }

    /// Creates a `POST` request, send it at `/login` and then return the response.
    pub async fn post_login<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `GET` request, send it at `/dashboard/` and then return the response.
    pub async fn get_dashboard(&self) -> Response {
        self.api_client
            .get(&format!("{}/dashboard/", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Returns the dashboard full response text.
    pub async fn get_dashboard_html(&self) -> String {
        self.get_dashboard().await.text().await.unwrap()
    }

    /// Creates a `GET` request, send it at `/dashboard/tickets/new` and then return the response.
    pub async fn get_create_tickets(&self) -> Response {
        self.api_client
            .get(&format!("{}/dashboard/tickets/new", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Returns the create tickets full response text.
    pub async fn get_create_tickets_html(&self) -> String {
        self.get_create_tickets().await.text().await.unwrap()
    }

    /// Creates a `POST` request, send it at `/dashboard/tickets/new` and then return the response.
    pub async fn post_tickets(&self, body: String) -> Response {
        self.api_client
            .post(&format!("{}/dashboard/tickets/new", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `GET` request, send it at `/dashboard/password` and then return the response.
    pub async fn get_change_password(&self) -> Response {
        self.api_client
            .get(&format!("{}/dashboard/password", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Returns the change password form full response text.
    pub async fn get_change_password_html(&self) -> String {
        self.get_change_password().await.text().await.unwrap()
    }

    /// Creates a `POST` request, send it at `/dashboard/password` and then return the response.
    pub async fn post_change_password<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/dashboard/password", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `GET` request, send it at `/dashboard/tickets` and then return the response.
    pub async fn get_see_tickets(&self) -> Response {
        self.api_client
            .get(&format!("{}/dashboard/tickets", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `GET` request, send it at `/dashboard/tickets/{id}` and then return the response.
    pub async fn get_see_ticket(&self, ticket_id: i32) -> Response {
        self.api_client
            .get(&format!(
                "{}/dashboard/tickets/{}",
                &self.address, ticket_id
            ))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Creates a `POST` request, send it at `/dashboard/logout` and then return the response.
    pub async fn post_logout(&self) -> Response {
        self.api_client
            .post(&format!("{}/dashboard/logout", &self.address))
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

    let application = Application::new(configuration.clone())
        .await
        .expect("Failed to create the application");
    let application_port = application.port();
    tokio::spawn(application.run_until_stopped());

    // Build the HTTP client.
    let client = reqwest::Client::builder()
        // Forbid to follow redirects.
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    // Create test application.
    let test_app = TestApp {
        db_pool: configuration.database.get_connection_pool(),
        address: format!("http://127.0.0.1:{}", application_port),
        api_client: client,
        test_user: TestUser::new(),
    };

    // Store the test user.
    test_app.test_user.store(&test_app.db_pool).await;

    test_app
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
