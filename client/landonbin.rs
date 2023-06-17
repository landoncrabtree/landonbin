use std::env;
use std::fs::File;
use std::io::Read;
use base64::{Engine as _, engine::general_purpose};
use reqwest::blocking::Client;
use serde_json::json;
use atty::Stream;
use clipboard::{ClipboardContext, ClipboardProvider};

fn b64encode(text_to_encode: &str) -> String {
    return general_purpose::STANDARD_NO_PAD.encode(text_to_encode.as_bytes());
}

fn post_data(content: &str, expiry: &str) {
    let data = json!({ "content": content, "expiry": expiry });
    let client = Client::new();
    let res = client
        .post("https://api.example.com/pastes")
        .header("Content-Type", "application/json")
        .header("X-API-Key", "my-api-key")
        .json(&data)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json_data: serde_json::Value = response.json().unwrap();
                if let Some(url) = json_data["url"].as_str() {
                    println!("{}", url);
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(url.to_string()).unwrap();
                } else {
                    println!("Raw Response: {}", json_data);
                }
            } else {
                let error_message: serde_json::Value = response.json().unwrap();
                eprintln!("Raw Response: {}", error_message);
            }
        }
        Err(e) => eprintln!("Request failed: {}", e),
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

