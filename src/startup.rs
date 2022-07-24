use crate::{
    configuration::Settings,
    routes::{create_ticket, health_check, home},
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
        // Get the connection pool for the Postgres database.
        let connection_pool = configuration.database.get_connection_pool();

        // Get the socket address and port.
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        // Create application data.
        let db_pool = web::Data::new(connection_pool);

        // Create the HTTP server.
        //
        // The HTTP server must be awaited or polled in order to start running.
        let server = HttpServer::new(move || {
            App::new()
                // Middleware.
                .wrap(TracingLogger::default())
                // Endpoints.
                .route("/", web::get().to(home))
                .route("/health_check", web::get().to(health_check))
                .route("/tickets", web::post().to(create_ticket))
                // Set application data.
                .app_data(db_pool.clone())
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
