use tessera::{configuration::get_configuration, startup::Application};

pub struct TestApp {
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
}

/// Build and then run the test application.
pub async fn spawn_test_app() -> TestApp {
    // Get the configuration values.
    let configuration = {
        let mut configuration =
            get_configuration().expect("Failed to get the configuration values");
        // Bind the socket address by triggering an OS scan using the port 0,
        // to find a random available port.
        configuration.application.port = 0;

        configuration
    };

    // Build the application.
    let application = Application::build(configuration).expect("Failed to build the application");

    // Get the port.
    let application_port = application.port();

    // Run the application.
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
    }
}
