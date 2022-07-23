use crate::{configuration::Settings, routes::home};
use actix_web::{dev::Server, web, App, HttpServer};

pub struct Application {
    server: Server,
}

impl Application {
    /// Build the application.
    pub fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Get the socket address.
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        // Create the HTTP server.
        //
        // The HTTP server must be awaited or polled in order to start running.
        let server = HttpServer::new(|| {
            App::new()
                // Endpoint.
                .route("/", web::get().to(home))
        })
        .bind(address)?
        .run();

        Ok(Self { server })
    }

    /// Run the application.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
