use std::fs;

use actix_web::{web, HttpResponse, Responder};
use crate::menu::{Category, Item, Menu, Restaurant};

pub async fn save_menu(
    form: web::Form<std::collections::HashMap<String, String>>,
) -> impl Responder {
    // Extract the id from the form data
    let id = match form.get("id") {
        Some(id) => id.clone(),
        None => return HttpResponse::BadRequest().body("Missing id"),
    };

    // Extract the restaurant name and contact
    let restaurant_name = match form.get("restaurant") {
        Some(name) => name.clone(),
        None => return HttpResponse::BadRequest().body("Missing restaurant name"),
    };

    let contact = form.get("contact").cloned().unwrap_or_default();
    let theme = form.get("theme").cloned().unwrap_or_default();

    // Initialize the categories vector
    let mut categories = Vec::new();

    // Since we allowed for 2 categories, iterate over 0..1
    for i in 0..2 {
        let category_name_key = format!("category_name_{}", i);
        if let Some(category_name) = form.get(&category_name_key) {
            if !category_name.is_empty() {
                let mut items = Vec::new();

                // For each category, iterate over 0..2 for items
                for j in 0..3 {
                    let item_name_key = format!("item_name_{}_{}", i, j);
                    let item_price_key = format!("item_price_{}_{}", i, j);
                    let item_description_key = format!("item_description_{}_{}", i, j);

                    if let Some(item_name) = form.get(&item_name_key) {
                        if !item_name.is_empty() {
                            let item_price = form.get(&item_price_key).cloned().unwrap_or_default();
                            let item_description = form.get(&item_description_key).cloned();

                            let item = Item {
                                name: item_name.clone(),
                                price: item_price,
                                description: item_description.filter(|d| !d.is_empty()),
                            };

                            items.push(item);
                        }
                    }
                }

                let category = Category {
                    name: category_name.clone(),
                    items,
                };

                categories.push(category);
            }
        }
    }

    let menu = Menu { categories };

    let restaurant = Restaurant {
        restaurant: restaurant_name,
        contact,
        menu,
        theme: Some(theme),
    };

    // Serialize the restaurant struct to JSON
    let json_data = match serde_json::to_string_pretty(&restaurant) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing JSON: {}", e);
            return HttpResponse::InternalServerError().body("Error serializing JSON");
        }
    };

    // Save the JSON data to the resources directory
    let file_path = format!("resources/{}.json", id);
    match fs::write(&file_path, json_data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error saving file: {}", e);
            return HttpResponse::InternalServerError().body("Error saving file");
        }
    };

    // Redirect the user to the /serve?id=X page
    let redirect_url = format!("/serve?id={}", id);
    HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish()
}
