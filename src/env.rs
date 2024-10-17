use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Vars {
    pub application_host: String,
    pub application_port: u16,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_database: String,
    pub rapid_api_key: String,
    pub rapid_api_host: String,
    pub native_x_api_key: String,
    pub native_x_bear_token: String,
}

impl Vars {
    pub fn load() -> Result<Self, std::io::Error> {
        Ok(Self {
            application_host: load_var("APPLICATION_HOST")?,
            application_port: load_var("PORT")?,
            postgres_user: load_var("POSTGRES_USER")?,
            postgres_password: load_var("POSTGRES_PASSWORD")?,
            postgres_host: load_var("POSTGRES_HOST")?,
            postgres_port: load_var("POSTGRES_PORT")?,
            postgres_database: load_var("POSTGRES_DATABASE")?,
            rapid_api_key: load_var("RAPID_X_API_KEY")?,
            rapid_api_host: load_var("RAPID_X_API_HOST")?,
            native_x_api_key: load_var("NATIVE_X_API_KEY")?,
            native_x_bear_token: load_var("NATIVE_X_BEARER_TOKEN")?,
        })
    }

    pub fn get_postgres_connection_uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        )
    }
}

fn load_var<T>(name: &str) -> Result<T, std::io::Error>
where
    <T as FromStr>::Err: Sync + Send + std::error::Error + 'static,
    T: FromStr,
{
    std::env::var(name)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error loading env var '{}': {}", name, e),
            )
        })
        .and_then(|value| {
            value.parse().map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error parsing env var '{}': {}", name, e),
                )
            })
        })
}