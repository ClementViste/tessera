use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

/// Representation of the application.
pub struct Application {
    server: Server,
}

impl Application {
    /// Creates the application.
    pub fn new() -> Result<Self, std::io::Error> {
        // Create the HTTP server.
        //
        // The HTTP server must be awaited or polled in order to start running.
        let server = HttpServer::new(|| {
            App::new()
                // Endpoint.
                .route("/", web::get().to(HttpResponse::Ok))
        })
        .bind("127.0.0.1:8000")?
        .run();

        Ok(Self { server })
    }

    /// Runs the application until stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
