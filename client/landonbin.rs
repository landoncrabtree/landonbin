use std::env;
use std::fs::File;
use std::io::Read;
use base64::encode;
use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut paste = String::new();
    let mut expiry = "Never".to_string();

    if args.len() < 2 {
        eprintln!("No text or file specified");
        return;
    }

    if args[1] == "--file" {
        if args.len() < 3 {
            eprintln!("No file specified");
            return;
        }
        let file_path = &args[2];
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
                return;
            }
        };
        if let Err(e) = file.read_to_string(&mut paste) {
            eprintln!("Failed to read file: {}", e);
            return;
        }
        paste = encode(&paste);
    } else if args[1] == "--text" {
        if args.len() < 3 {
            eprintln!("No text specified");
            return;
        }
        paste = encode(&args[2]);
    } else {
        eprintln!("Invalid argument");
        return;
    }

    if args.len() > 3 {
        expiry = args[3].to_string();
    }

    let data = json!({ "content": paste, "expiry": expiry });
    let client = Client::new();
    let res = client
        .post("https://api.example.com/pastes")
        .header("Content-Type", "application/json")
        .header("X-API-Key", "my-secret-api-key")
        .json(&data)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json_data: serde_json::Value = response.json().unwrap();
                let url = json_data["url"].as_str().unwrap();
                println!("Raw Response: {}", json_data);
                println!("Paste URL: {}", url);
            } else {
                let error_message: serde_json::Value = response.json().unwrap();
                eprintln!("Raw Response: {}", error_message);
            }
        }
        Err(e) => eprintln!("Request failed: {}", e),
    }
}

