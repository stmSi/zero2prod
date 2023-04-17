use std::net::TcpListener;
use std::process::exit;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Panic if we can't read configuration
    let configuration = match get_configuration() {
        Ok(config) => config,
        Err(err) => {
            // Print error message and exit with code 1
            eprintln!("Error reading configuration: {}", err);
            exit(1);
        }
    };

    let connection_string = configuration.database.connection_string();
    let connection_pool = match PgPool::connect_lazy(&connection_string) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error DB '{}': {}", connection_string, err);
            exit(1);
        }
    };

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding address '{}': {}", &address, err);
            exit(1);
        }
    };
    println!(
        "Server Running at: http://{}:{}",
        configuration.application.host, configuration.application.port
    );
    run(listener, connection_pool)?.await
}
