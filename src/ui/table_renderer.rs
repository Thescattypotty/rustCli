use console::style;
use prettytable::{Table, row};
use serde_json::Value;

pub struct TableRenderer;

impl TableRenderer {
    pub fn render_response(
        method: &str,
        url: &str,
        status: &str,
        headers: &reqwest::header::HeaderMap,
        body: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers_table = Table::new();
        headers_table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
        
        headers_table.add_row(row![FbBlue => "Request & Response Headers"]);
        headers_table.add_row(row![FbBlue => "Method", method]);
        headers_table.add_row(row![FbBlue => "URL", url]);
        headers_table.add_row(row![FbBlue => "Status", status]);

        for (key, value) in headers {
            headers_table.add_row(row![Fb => key.to_string(), 
                value.to_str().unwrap_or("Unable to display")]);
        }

        println!("\n{}", style("Headers Information").red().bold());
        headers_table.printstd();

        let mut body_table = Table::new();
        body_table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);
        body_table.add_row(row![FbBlue => "Response Body"]);
        
        if let Ok(json) = serde_json::from_str::<Value>(body) {
            body_table.add_row(row![FbYellow => serde_json::to_string_pretty(&json)?]);
        } else {
            body_table.add_row(row![FbYellow => body]);
        }

        println!("\n{}", style("Response Body").red().bold());
        body_table.printstd();

        Ok(())
    }
}