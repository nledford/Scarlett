use std::env;

use deadpool_postgres::{Manager, Pool};
use tokio_postgres::Config;

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

pub fn get_addr() -> String {
    let addr = match env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_e) => "0.0.0.0:8000".to_string(),
    };
    env::set_var("SERVER_HOST", addr.clone());

    addr
}
