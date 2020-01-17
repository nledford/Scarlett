use std::env;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, App, HttpServer};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::Config;

use scarlett_server::handlers;

fn create_pool() -> Pool {
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env::set_var("RUST_BACKTRACE", "full");

    env_logger::init();
    dotenv::dotenv().ok();

    let addr = match env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_e) => "0.0.0.0:8000".to_string(),
    };
    env::set_var("SERVER_HOST", addr.clone());

    let pool = create_pool();

    println!("Server running at {}", &addr);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        header::ORIGIN,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600)
                    .finish(),
            )
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // DEFAULT ROUTES **********************************************************************
            .service(handlers::index)
            .service(handlers::status)
            // PHOTOS ******************************************************************************
            .service(handlers::photos::get_photos)
            // SCAN PHOTOS *************************************************************************
            .service(handlers::scan_photos::run_scan)
            // STATS *******************************************************************************
            .service(handlers::stats::get_photos_stats)
            // TAGS ********************************************************************************
            .service(handlers::tags::get_tags)
            .service(handlers::tags::create_tag)
            // RESET SEED **************************************************************************
            .service(handlers::photos::reset_seed)
    })
    .bind(&addr)?
    .run()
    .await
}
