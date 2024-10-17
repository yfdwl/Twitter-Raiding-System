use actix_web::{dev::Server, web::Data, App, HttpServer};
use std::net::TcpListener;

use crate::{
    controllers::{Controller, RaidsController},
    env::Vars,
};

use crate::core::db::PgDb;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(vars: Vars) -> Result<Self, std::io::Error> {
        let postgres_uri = vars.get_postgres_connection_uri();
        println!("Connecting to Postgres: {}", postgres_uri);
        let pg_db = PgDb::new(&vars.get_postgres_connection_uri())
            .await
            .expect("Failed to connect to PostgreSQL.");
        let address = format!("{}:{}", vars.application_host, vars.application_port);
        let listener = TcpListener::bind(address).expect("Failed to bind address.");
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, pg_db)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, pg_pool: PgDb) -> Result<Server, std::io::Error> {
    let pg_db = Data::new(pg_pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pg_db.clone())
            .service(RaidsController::get_scope("/raids"))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
