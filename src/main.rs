use stakit_raids::env::Vars;
use stakit_raids::server::Application;

#[actix_web::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    println!("Loading environment variables...");
    let vars = Vars::load()?;
    let application = Application::build(vars).await?;
    let vars = Vars::load()?;

    println!("Starting API on port {}...", vars.application_port);
    application.run_until_stopped().await?;
    Ok(())
}
