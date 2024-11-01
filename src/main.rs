mod menu;
mod serve_menu;
mod save_menu;


use actix_files::Files;
use actix_web::{web, App, HttpServer};
use save_menu::save_menu;
use serve_menu::serve_menu;
use tera::Tera;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Tera templates
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    HttpServer::new(move || {
        App::new()
            // Add Tera to app data
            .app_data(web::Data::new(tera.clone()))
            // Serve static files
            .service(Files::new("/static", "web"))
            // Route for serving menu or creating menu
            .route("/serve", web::get().to(serve_menu))
            // Route for saving menu
            .route("/save_menu", web::post().to(save_menu))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

