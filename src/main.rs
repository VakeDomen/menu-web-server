use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use tera::{from_value, to_value, Context, Function, Tera, Value, Error};

// Data structures
#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    price: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Category {
    name: String,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
struct Menu {
    categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
struct Restaurant {
    restaurant: String,
    contact: String,
    theme: String,
    menu: Menu,
}

// Handler function
async fn serve_menu(
    query: web::Query<std::collections::HashMap<String, String>>,
    tmpl: web::Data<Tera>,
) -> impl Responder {
    // Get the 'id' parameter from the query string
    let id = match query.get("id") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Missing id parameter"),
    };

    // Build the file path to the JSON file
    let file_path = format!("resources/{}.json", id);

    // Read the JSON file
    let data = match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(_) => return HttpResponse::NotFound().body("File not found"),
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

    // Render the template
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

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Tera templates
    let mut tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    tera.register_function("load_style", make_load_style());

    HttpServer::new(move || {
        App::new()
            // Add Tera to app data
            .app_data(web::Data::new(tera.clone()))
            // Serve static files
            .service(Files::new("/static", "web"))
            // Route for serving menu
            .route("/serve", web::get().to(serve_menu))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn make_load_style() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value, Error> {
        match args.get("theme") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(theme) => Ok(to_value(format!("<link rel='stylesheet' href='/static/theme_{}.css'>", theme)).unwrap()),
                Err(_) => Ok(to_value("<link rel='stylesheet' href='/static/theme_olive.css'>").unwrap())
            },
            None => Ok(to_value("<link rel='stylesheet' href='/static/theme_olive.css'>").unwrap())
        }
    })
}