use std::env;
use std::fs::File;
use std::io::BufReader;

use deadpool_postgres::{Manager, Pool};
use rustls::{NoClientAuth, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};
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
pub fn load_ssl_keys() -> ServerConfig {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("ssl/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("ssl/key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}