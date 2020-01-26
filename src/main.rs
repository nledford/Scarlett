use std::env;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, App, HttpServer};

use scarlett_server::handlers;
use scarlett_server::utils::http_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env::set_var("RUST_BACKTRACE", "full");

    env_logger::init();
    dotenv::dotenv().ok();

    let addr = http_server::get_addr();
    let pool = http_server::create_pool();

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
            // COLLECTIONS *************************************************************************
            .service(handlers::collections::get_collections)
            .service(handlers::collections::get_collection)
            .service(handlers::collections::create_collection)
            .service(handlers::collections::update_collection)
            .service(handlers::collections::delete_collection)
            // DIRECTORY TREE **********************************************************************
            .service(handlers::directory_tree::get_tree)
            // ENTITIES ****************************************************************************
            .service(handlers::entity::get_entities)
            .service(handlers::entity::create_entity_simple)
            .service(handlers::entity::update_entity)
            .service(handlers::entity::delete_entity)
            // MEDIA *******************************************************************************
            .service(actix_files::Files::new("/media", "/photos").show_files_listing())
            .service(handlers::media::static_files)
            // PHOTOS ******************************************************************************
            .service(handlers::photos::get_photos)
            .service(handlers::photos::get_photo)
            .service(handlers::photos::update_photo)
            .service(handlers::photos::delete_photo)
            .service(handlers::photos::add_entity_to_photo)
            .service(handlers::photos::remove_entity_from_photo)
            .service(handlers::photos::add_tag_to_photo)
            .service(handlers::photos::remove_tag_from_photo)
            // SCAN PHOTOS *************************************************************************
            .service(handlers::scan_photos::run_scan)
            // STATS *******************************************************************************
            .service(handlers::stats::get_entity_stats)
            .service(handlers::stats::get_photos_stats)
            .service(handlers::stats::get_tag_stats)
            // TAGS ********************************************************************************
            .service(handlers::tags::get_tags)
            .service(handlers::tags::create_tag)
            .service(handlers::tags::update_tag)
            .service(handlers::tags::delete_tag)
            // WALLPAPER SIZES *********************************************************************
            .service(handlers::wallpapers::get_wallpaper_sizes)
            // RESET SEED **************************************************************************
            .service(handlers::photos::reset_seed)
    })
    .bind(&addr)?
    .run()
    .await
}
