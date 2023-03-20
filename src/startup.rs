use crate::{
    configuration::Settings,
    routes::{create_ticket, health_check},
};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Representation of the application.
pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    /// Creates the application.
    pub fn new(configuration: Settings) -> Result<Self, std::io::Error> {
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
                .route("/health_check", web::get().to(health_check))
                .route("/tickets/new", web::post().to(create_ticket))
        })
        .listen(listener)?
        .run();

        Ok(Self { server, port })
    }

    /// Returns the port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Runs the application until stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
