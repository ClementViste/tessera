use reqwest::{Client, Response};
use tessera::{configuration::get_configuration, startup::Application};

/// Representation of a test application.
pub struct TestApp {
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
}

/// Creates and then run the test application.
pub async fn create_and_run_test_app() -> TestApp {
    let configuration = {
        let mut configuration =
            get_configuration().expect("Failed to get the configuration values");
        // Find a random available port by triggering an OS scan using the port 0.
        configuration.application.port = 0;

        configuration
    };

    let application = Application::new(configuration).expect("Failed to create the application");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
    }
}
