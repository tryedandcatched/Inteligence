use crate::scanners::{scanner::ScannerTrait, scrapers::Scraper};

pub async fn scan() -> Scraper {
    let mut scraper = Scraper::new(
        String::from("Tass"),
        String::from("https://tass.com/"),
        vec![scraper::Selector::parse(".text-content").unwrap()],
        scraper::Selector::parse("a.news-preview").unwrap(),
        crate::scanners::scanner::Country::Russia,
        Some(String::from("https://tass.com")),
    );
    println!("Scraping tass");
    scraper.scan().await;
    scraper
}
