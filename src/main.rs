use tessera::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize telemetry.
    let subscriber = get_subscriber("tessera".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to get the configuration values");

    let application = Application::new(configuration)?;
    application.run_until_stopped().await?;

    Ok(())
}
