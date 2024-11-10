mod menu;
mod serve_menu;
mod save_menu;
mod parse_image;


use actix_files::Files;
use actix_web::{web, App, HttpServer};
use save_menu::save_menu;
use serve_menu::serve_menu;
use parse_image::parse_image;
use tera::Tera;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();
    // Initialize Tera templates
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(Files::new("/static", "web"))
            .route("/serve", web::get().to(serve_menu))
            .route("/save_menu", web::post().to(save_menu))
            .route("/parse_image", web::post().to(parse_image))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
