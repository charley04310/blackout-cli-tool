use box_drawing::light;
use clap::Parser;
use directories::ProjectDirs;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = "Blackout is a tool that helps you to remember CLI commands"
)] // Read from `Cargo.toml`
struct Cli {
    /// The pattern to look for
    #[arg(short, long, help = "The technology you want to use")]
    technology: Option<String>,
    /// The path to the file to read
    #[arg(short, long, help = "The action you want to do")]
    action: Option<String>,
    // Reset IP address
    #[arg(short, long, help = "Reset your Ip address")]
    reset_ip: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    // Getting the API key from the environment
    let api_key = initialize_config()?;

    if args.reset_ip {
        reset_ip_adress(&api_key).await?;
        return Ok(());
    }

    // If user doesn't want to reset Ip. He  must have at least have technology and question
    let result = match (args.technology.as_deref(), args.action.as_deref()) {
        (Some(technology), Some(action)) => {
            // Constructing the message using technology and question
            let message = format!("You are an expert in computer science. Give me the code me using {} how to {}. Have short answer with only code example", technology, action);

            // // Sending the cURL request to the API
            send_curl_request(&api_key, &message).await?;

            Ok(())
        }
        (None, Some(_)) => Err("Technology is missing."),
        (Some(_), None) => Err("Question is missing."),
        (None, None) => Err("Both technology and question are missing."),
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }

    Ok(())
}

async fn send_curl_request(api_key: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.pawan.krd/unfiltered/v1/chat/completions";

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "max_tokens": 100,
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant in computer science."
            },
            {
                "role": "user",
                "content": message
            }
        ]
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;

    let code_blocks = extract_code_blocks(&response_text);


    for block in code_blocks {
        println!("{}", light::HORIZONTAL.repeat(block.len() + 2));
        println!(" {} ", block);
        println!("{}", light::HORIZONTAL.repeat(block.len() + 2));
    }

    Ok(())
}

async fn reset_ip_adress(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {


    let client = Client::new();
    let url = "https://api.pawan.krd/resetip";


    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let response_text = response.text().await?;


    println!("{}", response_text);

    Ok(())
}

fn extract_code_blocks(json_response: &str) -> Vec<String> {
    let response: Value = serde_json::from_str(json_response).unwrap();
    let content = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap();

    let mut code_blocks: Vec<_> = Vec::new();
    let lines = content.lines();

    let mut in_code_block = false;
    let mut code_block = String::new();

    for line in lines {
        if line.starts_with("```") {
            if in_code_block {
                in_code_block = false;
                code_blocks.push(code_block.trim().to_string());
                code_block.clear();
            } else {
                in_code_block = true;
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        }
    }

    code_blocks
}

fn read_api_key(config_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(config_path)?;
    let api_key = file_content.trim().to_owned();
    Ok(api_key)
}

fn initialize_config() -> Result<String, Box<dyn std::error::Error>> {
    let project_path = ProjectDirs::from("fr", "charley", "blackout").unwrap();

    let config_dir = project_path.config_dir();
    let hook_path_exists: bool = Path::new(&config_dir).exists();
    let config_path = config_dir.join("config");

    let mut api_key: String = String::new();

    if !hook_path_exists {
        fs::create_dir_all(&config_dir)?;

        println!("Welcome to BLACKOUT Tool! Enter your API key:");
        io::stdin().read_line(&mut api_key)?;

        let mut file = fs::File::create(&config_path)?;
        write!(file, "{}", api_key.trim())?;

        api_key = read_api_key(&config_path.to_string_lossy())?;
    } else {
        // Retrieve the API key from the configuration file
        api_key = read_api_key(&config_path.to_string_lossy())?;
    }

    Ok(api_key)
}

// Here is CHATGTP Official client

// use chatgpt::{prelude::*, types::CompletionResponse};
// use clap::Parser;
// use dotenv::dotenv;

// /// Search for a pattern in a file and display the lines that contain it.
// #[derive(Parser)]
// #[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
// struct Cli {
//     /// The pattern to look for
//     #[arg(short, long)]
//     technology: Option<String>,
//     /// The path to the file to read
//     #[arg(short, long)]
//     question: Option<String>,
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let args = Cli::parse();
//     dotenv().ok(); // This line loads the environment variables from the ".env" file.

//     // We must have at least technology and question
//     let result = match (args.technology.as_deref(), args.question.as_deref()) {
//         (Some(technology), Some(question)) => {
//             println!("Value for technology: {}", technology);
//             println!("Value for config: {}", question);

//             // Getting the API key from the environment
//             let key = std::env::var("OPENAI_API_KEY")
//                 .expect("OPENAI_API_KEY is not set in the environment.");

//             // Creating a new ChatGPT client.
//             // Note that it requires an API key, and uses
//             // tokens from your OpenAI API account balance.
//             let client = ChatGPT::new(key)?;

//             // Constructing the message using technology and question
//             let message = format!("You are an expert in computer science. Explain me using {} how to {}. Write only the code", technology, question);

//             // Sending a message and getting the completion
//             let response: CompletionResponse = client.send_message(&message).await?;
//             println!("Response: {}", response.message().content);

//             Ok(())
//         }
//         (None, Some(_)) => Err("Technology is missing."),
//         (Some(_), None) => Err("Question is missing."),
//         (None, None) => Err("Both technology and question are missing."),
//     };

//     if let Err(error) = result {
//         eprintln!("Error: {}", error);
//     }

//     Ok(())
// }
