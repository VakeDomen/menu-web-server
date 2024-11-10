use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};
use tera::{Context, Tera};
use serde_json::Value;

use crate::menu::Restaurant;


pub async fn serve_menu(
    query: web::Query<HashMap<String, String>>,
    tmpl: web::Data<Tera>,
) -> impl Responder {
    // Get the 'id' parameter from the query string
    let id = match query.get("id") {
        Some(id) => id.clone(),
        None => return HttpResponse::BadRequest().body("Missing id parameter"),
    };

    // Build the file path to the JSON file
    let file_path = format!("resources/{}.json", id);

    // Check if the 'edit' parameter is present
    if let Some(edit_hash) = query.get("edit") {
        // Secret key for hashing
        let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| "default_secret".to_string());

        // Compute MD5 hash of the id combined with the secret key
        let combined = format!("{}{}", id, secret_key);
        let computed_hash = format!("{:x}", md5::compute(combined));

        if *edit_hash == computed_hash {
            // The edit hash is valid; serve the create_menu.html with existing data

            // Try to read the JSON file
            let data = match fs::read_to_string(&file_path) {
                Ok(data) => data,
                Err(_) => {
                    // File not found; start with empty data
                    let mut context = Context::new();
                    context.insert("id", &id);
                    context.insert("menu_data", &serde_json::json!({
                        "restaurant": "",
                        "contact": "",
                        "theme": "",
                        "menu": {
                            "categories": []
                        }
                    }));

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

            // Deserialize the JSON data into serde_json::Value
            let menu_data_value: Value = match serde_json::from_str(&data) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    return HttpResponse::InternalServerError().body("Error parsing JSON");
                }
            };

            // Create a Tera context and insert the menu data
            let mut context = Context::new();
            context.insert("id", &id);
            context.insert("menu_data", &menu_data_value);

            // Render the create_menu.html template
            let rendered = match tmpl.render("create_menu.html", &context) {
                Ok(rendered) => rendered,
                Err(e) => {
                    eprintln!("Template rendering error: {}", e);
                    return HttpResponse::InternalServerError().body("Template rendering error");
                }
            };

            // Return the rendered HTML response
            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered);
        } else {
            // Invalid edit hash
            return HttpResponse::Unauthorized().body("Invalid edit hash");
        }
    }

    // No 'edit' parameter; serve the menu.html or create_menu.html if file not found

    // Try to read the JSON file
    let data = match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(_) => {
            // File not found; render the create_menu page
            let mut context = Context::new();
            context.insert("id", &id);
            context.insert("menu_data", &serde_json::json!({
                "restaurant": "",
                "contact": "",
                "theme": "",
                "menu": {
                    "categories": []
                }
            }));

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

    // Render the menu.html template
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
