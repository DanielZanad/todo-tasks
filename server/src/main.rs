use std::net::TcpListener;

use crate::{db::configuration::get_configuration, env::get_env_var};

pub mod db;
pub mod env;
pub mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = get_env_var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a valid u16");
    let connection = get_configuration().await.unwrap();
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, connection)?.await
}
