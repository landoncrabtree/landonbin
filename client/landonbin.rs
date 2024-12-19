use std::env;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose};
use reqwest::blocking::Client;
use serde_json::json;
use atty::Stream;
use clipboard::{ClipboardContext, ClipboardProvider};

// Function to encode text to base64
fn b64encode(text_to_encode: &str) -> String {
    return general_purpose::STANDARD_NO_PAD.encode(text_to_encode.as_bytes());
}

// Function to post data to the API
fn post_data(content: &str, expiry: &str) {
    let data = json!({ "content": content, "expiry": expiry });
    let client = Client::new();
    let res = client
        .post("https://api.example.com/pastes")
        .header("Content-Type", "application/json")
        .header("X-API-Key", "")
        .json(&data)
        .send();
    match res {
        Ok(response) => {
            eprintln!("Response Status: {}", response.status());
            eprintln!("Response Headers: {:?}", response.headers());

            // Check if the response is successful before consuming it
            if response.status().is_success() {
                let raw_response = response
                    .text()
                    .unwrap_or_else(|_| "Failed to retrieve body".to_string());

                match serde_json::from_str::<serde_json::Value>(&raw_response) {
                    Ok(json_data) => {
                        if let Some(url) = json_data["url"].as_str() {
                            println!("{}", url);
                            if !can_copy_clipboard() {
                                return;
                            }
                            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                            ctx.set_contents(url.to_string()).unwrap();
                        } else {
                            println!("Unexpected Response Format: {}", json_data);
                        }
                    }
                    Err(_) => {
                        eprintln!("Invalid JSON Response: {}", raw_response);
                    }
                }
            } else {
                let raw_response = response
                    .text()
                    .unwrap_or_else(|_| "Failed to retrieve body".to_string());
                eprintln!("Error Response: {}", raw_response);
            }
        }
        Err(e) => eprintln!("Request failed: {}", e),
    }
}

// Retrieve the COPY_CLIPBOARD environment variable
fn can_copy_clipboard() -> bool {
    let copy_clipboard = match env::var("COPY_CLIPBOARD") {
        Ok(val) => val,
        Err(_e) => "FALSE".to_string(),
    };

    return string_to_bool(&copy_clipboard);
}

// Helper function to convert string to bool
fn string_to_bool(s: &str) -> bool {
    match s {
        "TRUE" => true,
        "True" => true,
        "true" => true,
        "1" => true,
        "t" => true,
        "T" => true,
        "FALSE" => false,
        "False" => false,
        "false" => false,
        "0" => false,
        "f" => false,
        "F" => false,
        _ => false,
    }
}

fn print_usage() {
    println!("USAGE: landonbin [OPTIONS] [EXPIRY]");
    println!("Options:");
    println!("  --file FILE_PATH: Specify a file to upload");
    println!("  --text TEXT: Specify text to upload");
    println!("  expiry: Specify the expiry of the paste (default: Never)");
    println!("");
    println!("Examples:");
    println!("  landonbin --file /home/user/hello.txt 1h");
    println!("  landonbin --text \"Hello, World!\"");
    println!("  python3 do_something.py | landonbin 7d");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content = String::new();
    let mut expiry = "Never".to_string();

    if !atty::is(Stream::Stdin) {
        std::io::stdin().read_to_string(&mut content).unwrap();
        content = b64encode(&content);
        if args.len() >= 2 {
            expiry = args[1].to_string();
        }
        return post_data(&content, &expiry);
    }

    if args.len() < 2 {
        print_usage();
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
        if let Err(e) = file.read_to_string(&mut content) {
            eprintln!("Failed to read file: {}", e);
            return;
        }
        content = b64encode(&content);
    } else if args[1] == "--text" {
        if args.len() < 3 {
            eprintln!("No text specified");
            return;
        }
        content = b64encode(&args[2]);
    } else {
        print_usage();
        return;
    }

    if args.len() > 3 {
        expiry = args[3].to_string();
    }

    post_data(&content, &expiry);
    return;
}

