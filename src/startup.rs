use crate::{
    authentication::reject_anonymous_users,
    configuration::Settings,
    routes::{
        change_password, change_password_form, create_ticket, create_ticket_form, dashboard,
        health_check, home, login, login_form, logout, register, register_form, see_ticket,
        see_tickets,
    },
};
use actix_files::Files;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie, dev::Server, web, App, HttpServer};
use actix_web_flash_messages::{storage, FlashMessagesFramework};
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Representation of the HMAC secret.
#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

/// Representation of the application.
pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    /// Creates the application.
    pub async fn new(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = configuration.database.get_connection_pool();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        // Create application data.
        let db_pool = web::Data::new(connection_pool);
        let hmac_secret = web::Data::new(HmacSecret(configuration.application.hmac_secret));
        let message_store = storage::CookieMessageStore::builder(cookie::Key::from(
            hmac_secret.0.expose_secret().as_bytes(),
        ))
        .build();
        let message_framework = FlashMessagesFramework::builder(message_store).build();
        let redis_store = RedisSessionStore::new(configuration.redis_uri.expose_secret()).await?;

        // Create the HTTP server.
        //
        // The HTTP server must be awaited or polled in order to start running.
        let server = HttpServer::new(move || {
            App::new()
                // Serve static files.
                .service(Files::new("/static", "static"))
                // Middlewares.
                .wrap(TracingLogger::default())
                .wrap(message_framework.clone())
                .wrap(SessionMiddleware::new(
                    redis_store.clone(),
                    cookie::Key::from(hmac_secret.0.expose_secret().as_bytes()),
                ))
                // Endpoints.
                .route("/", web::get().to(home))
                .route("/health_check", web::get().to(health_check))
                .route("/register", web::get().to(register_form))
                .route("/register", web::post().to(register))
                .route("/login", web::get().to(login_form))
                .route("/login", web::post().to(login))
                .service(
                    web::scope("/dashboard")
                        .wrap(from_fn(reject_anonymous_users))
                        .route("/", web::get().to(dashboard))
                        .route("/tickets/new", web::get().to(create_ticket_form))
                        .route("/tickets/new", web::post().to(create_ticket))
                        .route("/tickets", web::get().to(see_tickets))
                        .route("/tickets/{id}", web::get().to(see_ticket))
                        .route("/password", web::get().to(change_password_form))
                        .route("/password", web::post().to(change_password))
                        .route("/logout", web::post().to(logout)),
                )
                // Set application data.
                .app_data(db_pool.clone())
                .app_data(hmac_secret.clone())
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
