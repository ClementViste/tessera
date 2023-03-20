use tessera::startup::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let application = Application::new()?;
    application.run_until_stopped().await?;

    Ok(())
}
