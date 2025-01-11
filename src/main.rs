use clap::Parser;
use dialoguer::{Input, Select};
use std::{collections::HashMap, fs};
use rustcli::{models::HttpFile, handlers::RequestHandler};
use console::style;
use glob::glob;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    file: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    if let Some(patterns) = cli.file {
        let mut files_count = 0;
        
        for pattern in patterns {
            let paths = glob(&pattern)?;
            
            for entry in paths {
                match entry {
                    Ok(path) => {
                        println!("\n{}", style(format!("Processing file: {}", path.display())).cyan().bold());
                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                match HttpFile::parse(&content) {
                                    Ok(http_file) => {
                                        RequestHandler::execute_request(
                                            &http_file.url,
                                            &http_file.method,
                                            http_file.headers,
                                            http_file.body,
                                        ).await?;
                                        files_count += 1;
                                    },
                                    Err(e) => println!("{}", style(format!("Error parsing file {}: {}", path.display(), e)).red()),
                                }
                            },
                            Err(e) => println!("{}", style(format!("Error reading file {}: {}", path.display(), e)).red()),
                        }
                    },
                    Err(e) => println!("{}", style(format!("Error with path: {}", e)).red()),
                }
            }
        }
        
        println!("\n{}", style(format!("Processed {} .http files", files_count)).green().bold());
    } else {
        loop {
            let url: String = if let Some(ref url) = cli.url {
                url.clone()
            } else {
                let input = Input::<String>::new()
                    .with_prompt("Enter URL (empty to exit)")
                    .allow_empty(true)
                    .interact()?;
                
                if input.is_empty() {
                    println!("{}", style("Goodbye!").cyan().bold());
                    break;
                }
                input
            };

            let methods = vec!["GET", "POST", "PUT", "DELETE"];
            let method = Select::new()
                .with_prompt("Select HTTP method")
                .items(&methods)
                .default(0)
                .interact()?;

            let mut headers: HashMap<String, String> = HashMap::new();
            if confirm("Add headers?")? {
                loop {
                    let key: String = Input::new()
                        .with_prompt("Header key (empty to finish)")
                        .allow_empty(true)
                        .interact()?;
                    
                    if key.is_empty() {
                        break;
                    }

                    let value: String = Input::new()
                        .with_prompt("Header value")
                        .interact()?;

                    headers.insert(key, value);
                }
            }

            let body = if methods[method] != "GET" && confirm("Add request body?")? {
                Some(Input::<String>::new()
                    .with_prompt("Enter request body (JSON)")
                    .interact()?)
            } else {
                None
            };

            RequestHandler::execute_request(
                &url,
                methods[method],
                headers,
                body,
            ).await?;
        
            // Ask if user wants to make another request
            if !cli.file.is_some() && !cli.url.is_some() {
                if !confirm("Make another request?")? {
                    println!("{}", style("Goodbye!").cyan().bold());
                    break;
                }
                println!("{}", style("\n--- New Request ---").cyan().bold());
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn confirm(message: &str) -> Result<bool, std::io::Error> {
    Select::new()
        .with_prompt(message)
        .items(&["Yes", "No"])
        .default(0)
        .interact()
        .map(|i| i == 0)
}
