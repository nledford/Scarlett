use std::env;

use actix_tls::openssl::{SslAcceptor, SslAcceptorBuilder};
use deadpool_postgres::{Manager, Pool};
use openssl::ssl::{SslFiletype, SslMethod};
use tokio_postgres::Config;

/// Builds a Postgresql data pool using environment variables.
///
/// The environment variables are `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB`, and `POSTGRES_HOST`.
pub fn create_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.user(&env::var("POSTGRES_USER").expect("POSTGRES_USER environment variable not set"));
    cfg.password(
        &env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD environment variable not set"),
    );
    cfg.dbname(&env::var("POSTGRES_DB").expect("POSTGRES_DB environment variable not set"));
    cfg.host(&env::var("POSTGRES_HOST").expect("POSTGRES_HOST environment variable not set"));

    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    Pool::new(mgr, 16)
}

/// Builds an address string from the `SERVER_HOST` environment variable if it is set.
/// Otherwise returns a default value of `0.0.0.0:8000`.
pub fn get_addr() -> String {
    let addr = match env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_e) => "0.0.0.0:8000".to_string(),
    };
    env::set_var("SERVER_HOST", addr.clone());

    addr
}

/// Loads `key.pem` and `cert.pem` from the `/ssl` directory
pub fn load_ssl_keys() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/ssl/key.pem", SslFiletype::PEM)
        .unwrap();

    builder.set_certificate_chain_file("/ssl/cert.pem").unwrap();

    builder
}
