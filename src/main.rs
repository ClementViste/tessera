use tessera::{configuration::get_configuration, startup::Application};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get the configuration values");

    let application = Application::new(configuration)?;
    application.run_until_stopped().await?;

    Ok(())
}
