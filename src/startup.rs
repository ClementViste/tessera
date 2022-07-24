use crate::{
    configuration::Settings,
    routes::{health_check, home},
};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    /// Build the application.
    pub fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Get the socket address and port.
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        // Create the HTTP server.
        //
        // The HTTP server must be awaited or polled in order to start running.
        let server = HttpServer::new(|| {
            App::new()
                // Middleware.
                .wrap(TracingLogger::default())
                // Endpoints.
                .route("/", web::get().to(home))
                .route("/health_check", web::get().to(health_check))
        })
        // Bind the socket address.
        .listen(listener)?
        .run();

        Ok(Self { server, port })
    }

    /// Return the port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Run the application.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
