use std::net::TcpListener;
use std::process::exit;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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
    let connection_pool = match PgPool::connect(&connection_string).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error DB '{}': {}", connection_string, err);
            exit(1);
        }
    };

    let address = format!("127.0.0.1:{}", configuration.applicaton_port);
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding address '{}': {}", &address, err);
            exit(1);
        }
    };
    run(listener, connection_pool)?.await
}
