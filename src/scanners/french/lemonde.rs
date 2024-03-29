use crate::scanners::{scanner::ScannerTrait, scrapers::Scraper};

pub async fn scan() -> Scraper {
    let mut scraper = Scraper::new(
        String::from("Lemonde"),
        String::from("https://www.lemonde.fr/en/politics/"),
        vec![scraper::Selector::parse(".article__paragraph").unwrap()],
        vec![scraper::Selector::parse(".teaser__link").unwrap()],
        crate::scanners::scanner::Country::French,
        None,
        None,
    );
    scraper.scan().await;
    scraper
}
