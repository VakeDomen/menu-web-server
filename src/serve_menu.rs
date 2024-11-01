use actix_web::{web, HttpResponse, Responder};
use std::fs;
use tera::{Context, Tera};

use crate::menu::Restaurant;


// Modified serve_menu handler
pub async fn serve_menu(
    query: web::Query<std::collections::HashMap<String, String>>,
    tmpl: web::Data<Tera>,
) -> impl Responder {
    // Get the 'id' parameter from the query string
    let id = match query.get("id") {
        Some(id) => id.clone(),
        None => return HttpResponse::BadRequest().body("Missing id parameter"),
    };

    // Build the file path to the JSON file
    let file_path = format!("resources/{}.json", id);

    // Try to read the JSON file
    let data = match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(_) => {
            // File not found; render the create_menu page
            let mut context = Context::new();
            context.insert("id", &id);

            let rendered = match tmpl.render("create_menu.html", &context) {
                Ok(rendered) => rendered,
                Err(e) => {
                    eprintln!("Template rendering error: {}", e);
                    return HttpResponse::InternalServerError().body("Template rendering error");
                }
            };

            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered);
        }
    };

    // Parse the JSON data into the Restaurant struct
    let restaurant: Restaurant = match serde_json::from_str(&data) {
        Ok(restaurant) => restaurant,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return HttpResponse::InternalServerError().body("Error parsing JSON");
        }
    };

    // Create a Tera context and insert the restaurant data
    let mut context = Context::new();
    context.insert("restaurant", &restaurant);

    // Render the menu template
    let rendered = match tmpl.render("menu.html", &context) {
        Ok(rendered) => rendered,
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            return HttpResponse::InternalServerError().body("Template rendering error");
        }
    };

    // Return the rendered HTML response
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}
