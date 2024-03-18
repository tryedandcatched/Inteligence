use crate::scanners::{scanner::ScannerTrait, scrapers::Scraper};

pub async fn scan() -> Scraper {
    let mut scraper = Scraper::new(
        String::from("Leparisien"),
        String::from("https://www.leparisien.fr/politique/"),
        vec![scraper::Selector::parse(".lp-card-article__link").unwrap()],
        vec![
            scraper::Selector::parse(".article-section").unwrap(),
            scraper::Selector::parse(".margin_bottom_subheadline").unwrap(),
        ],
        crate::scanners::scanner::Country::French,
        Some(String::from("https:")),
        None,
    );
    scraper.scan().await;
    scraper
}
