use newsletter_backend_template::startup::Application;
use newsletter_backend_template::telemetry::init_subscriber;
use newsletter_backend_template::{configuration::get_configuration, telemetry::get_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
