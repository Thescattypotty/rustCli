use std::collections::HashMap;
use chrono::format::{DelayedFormat, StrftimeItems};
use reqwest::{self, header::HeaderMap, Client, Response};
use crate::ui::table_renderer::TableRenderer;
use indicatif::ProgressBar;
use std::time::Duration;
use console::style;
use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;

pub struct RequestHandler;

impl RequestHandler {
    pub async fn execute_request(
        url: &str,
        method: &str,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", style("\nSending request...").cyan());
        println!("URL: {}", style(url).yellow());
        println!("Method: {}", style(method).yellow());

        let spinner: ProgressBar = ProgressBar::new_spinner();
        spinner.set_message("Executing request...");
        spinner.enable_steady_tick(Duration::from_millis(100));

        let client: Client = reqwest::Client::new();
        let mut request = match method {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => return Err("Unsupported HTTP method".into()),
        };

        for (key, value) in headers {
            request = request.header(key, value);
        }

        if let Some(body_content) = body {
            request = request.body(body_content);
        }

        let response: Response = request.send().await?;
        
        let status: String = response.status().to_string();
        let headers: HeaderMap = response.headers().clone();

        let body: String = response.text().await?;
        spinner.finish_and_clear();

        TableRenderer::render_response(method, url, &status, &headers, &body)?;
        
        Self::save_response(method, url, &status, &headers, &body, None)?;
        
        println!("\n{}", style("Request completed! Results saved to output.http").green().bold());
        Ok(())
    }

    fn save_response(
        method: &str,
        url: &str,
        status: &str,
        headers: &HeaderMap,
        body: &str,
        source_file: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp: DelayedFormat<StrftimeItems<'_>> = Local::now().format("%Y-%m-%d %H:%M:%S");
        let mut file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open("output.http")?;

        writeln!(file, "\n=== Request {} ===", timestamp)?;
        if let Some(source) = source_file {
            writeln!(file, "Source File: {}", source)?;
        }
        writeln!(file, "Method: {}", method)?;
        writeln!(file, "URL: {}", url)?;
        writeln!(file, "Status: {}", status)?;
        
        writeln!(file, "\n--- Headers ---")?;
        for (key, value) in headers {
            writeln!(file, "{}: {}", key, value.to_str().unwrap_or("Unable to display"))?;
        }
        
        writeln!(file, "\n--- Body ---")?;
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
            writeln!(file, "{}", serde_json::to_string_pretty(&json)?)?;
        } else {
            writeln!(file, "{}", body)?;
        }
        
        writeln!(file, "\n=== End Request ===\n")?;
        Ok(())
    }
}
