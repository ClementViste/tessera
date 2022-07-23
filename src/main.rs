use tessera::{configuration::get_configuration, startup::Application};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Get the configuration values.
    let configuration = get_configuration().expect("Failed to get the configuration values");

    // Build the application.
    let application = Application::build(configuration)?;

    // Run the application.
    application.run_until_stopped().await?;

    Ok(())
}
