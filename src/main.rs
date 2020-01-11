use std::env;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Client, Manager, Pool, PoolError};
use scarlett_server::models::db::PhotosAll;
use tokio_postgres::Config;

use scarlett_server::models::errors;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

// test query and handler
async fn all_photos(pool: &Pool) -> Result<Vec<PhotosAll>, PoolError> {
    let client: Client = pool.get().await?;
    let stmt = client.prepare("SELECT * FROM photos_all").await?;
    let rows = client.query(&stmt, &[]).await?;

    let photos = rows
        .into_iter()
        .map(PhotosAll::from_row)
        .collect::<Vec<PhotosAll>>();

    Ok(photos)
}

#[get("/photos")]
async fn get_photos(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = all_photos(&pool).await;

    match res {
        Ok(photos) => Ok(HttpResponse::Ok().json(photos)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}

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
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_photos)
    })
    .bind(&addr)?
    .run()
    .await
}
