use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // URL of Crates.io
    let url = "https://crates.io/crates";

    // Fetch the HTML content of the page
    let response = reqwest::get(url).await?.text().await?;

    // Parse the HTML using `scraper`
    let document = Html::parse_document(&response);

    // Define a selector to find crate names
    let selector = Selector::parse("a[href]").unwrap();

    println!("Top crates on Crates.io:");
    for element in document.select(&selector) {
        // Extract the text content from each selected element
        if let Some(crate_name) = element.text().next() {
            println!("{}", crate_name);
        }
    }

    Ok(())
}

