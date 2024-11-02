use std::env;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;

use crate::menu::Restaurant;

#[derive(Deserialize)]
pub struct ParseImageRequest {
    image: String,
}

pub async fn parse_image(
    parse_request: web::Json<ParseImageRequest>,
) -> impl Responder {
    // Extract the base64 image
    let base64_image = &parse_request.image;

    // Prepare the request payload for MistralAI
    let prompt = "Iz podanih slik menija restavracije/bara prepiši meni in scene. Tvoj odgovor naj bo meni v podani JSON obliki: {\"restaurant\": \"\", \"contact\": \"\", \"menu\": {\"categories\": [{\"name\": \"\", \"items\": [{\"name\": \"\", \"price\": \"\", \"description\": \"\"}]}]}}";
    
    let request_body = json!({
        "model": "pixtral-12b-2409",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": format!("data:image/jpeg;base64,{}", base64_image)
                    }
                ]
            }
        ]
    });

    // Get the MistralAI API key from an environment variable
    let api_key = env::var("MISTRAL_TOKEN").expect("NO MISTRAL TOKEN");

    // Send the request to MistralAI
    let client = Client::new();
    let response = match client
        .post("https://api.mistral.ai/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Error sending request to MistralAI: {}", e);
            return HttpResponse::InternalServerError().body("Error communicating with MistralAI");
        }
    };

    if !response.status().is_success() {
        eprintln!("MistralAI API error: {}", response.status());
        return HttpResponse::InternalServerError().body("MistralAI API error");
    }
    

    // Parse the response
    let resp_json: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing MistralAI response: {}", e);
            return HttpResponse::InternalServerError().body("Error parsing MistralAI response");
        }
    };

    // Extract the assistant's message
    let assistant_message = &resp_json["choices"][0]["message"]["content"];
    let assistant_text = assistant_message.as_str().unwrap_or("");

    let assistant_text = assistant_text.replace("```json", "");
    let assistant_text = assistant_text.replace("```", "");
    let assistant_text = format!("{}{}", "{", assistant_text.split_once("{").unwrap().1);

    // Parse the assistant's message as JSON
    let parsed_data: Restaurant = match serde_json::from_str(&assistant_text) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing assistant message as JSON: {}", e);
            return HttpResponse::InternalServerError().body("Error parsing assistant message");
        }
    };
    HttpResponse::Ok().json(parsed_data)
}
