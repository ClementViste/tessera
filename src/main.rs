use tessera::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize telemetry.
    let subscriber = get_subscriber("tessera".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Get the configuration values.
    let configuration = get_configuration().expect("Failed to get the configuration values");

    // Build the application.
    let application = Application::build(configuration)?;

    // Run the application.
    application.run_until_stopped().await?;

    Ok(())
}
