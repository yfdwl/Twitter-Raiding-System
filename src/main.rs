use stakit_raids::core::xapis::quote_users::get_replying_users;
// use stakit_raids::core::xapis::retweeted_users::get_retweeted_users;
use stakit_raids::env::Vars;
use stakit_raids::server::Application;

#[actix_web::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    // let _ = get_retweeted_users("1846574339434447299").await;
    let _ = get_replying_users("1846574339434447299").await;
    // let _ = get_replying_users("1847028043128136067").await;

    println!("Loading environment variables...");
    let vars = Vars::load()?;
    let application = Application::build(vars).await?;
    let vars = Vars::load()?;

    println!("Starting API on port {}...", vars.application_port);
    application.run_until_stopped().await?;
    Ok(())
}
