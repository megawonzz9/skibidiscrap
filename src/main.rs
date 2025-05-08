use reqwest;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Pobierz argumenty z linii poleceń
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Użycie: {} <URL>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    extract_visible_text(url).await?;
    Ok(())
}

async fn extract_visible_text(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    // Selektory: <p> i wszystkie nagłówki <h1> do <h6>
    let selectors = vec!["p", "h1", "h2", "h3", "h4", "h5", "h6"];

    for tag in selectors {
        let selector = Selector::parse(tag).unwrap();
        for element in document.select(&selector) {
            let text = element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            if !text.is_empty() {
                println!("{}", text);
            }
        }
    }

    Ok(())
}
