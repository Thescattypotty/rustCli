use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpFile {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpFile {
    pub fn parse(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines = content.lines();
        let first_line = lines.next().ok_or("Empty file")?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        let mut url: String = String::new();
        let mut method: String = String::new();
        let mut headers:HashMap<String, String> = HashMap::new();
        let mut body: Option<String> = None;
        let mut reading_body: bool = false;
        
        if parts.len() >= 2 {
            url = parts[0].to_string();
            method = parts[1].to_string();
            
            for part in parts[2..].iter() {
                if let Some(header) = part.strip_prefix("--") {
                    let kv: Vec<&str> = header.splitn(2, '=').collect();
                    if kv.len() == 2 {
                        headers.insert(kv[0].to_string(), kv[1].to_string());
                    }
                }
            }
        }

        for line in lines {
            if line.starts_with("--body") {
                reading_body = true;
                continue;
            }
            
            if reading_body {
                body = Some(line.trim().to_string());
                break;
            }
        }

        Ok(HttpFile {
            url,
            method,
            headers,
            body,
        })
    }
}