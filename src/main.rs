use tessera::startup::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Build the application.
    let application = Application::build()?;

    // Run the application.
    application.run_until_stopped().await?;

    Ok(())
}
