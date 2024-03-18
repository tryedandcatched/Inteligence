use crate::scanners::{scanner::ScannerTrait, scrapers::Scraper};

pub async fn scan() -> Scraper {
    let mut scraper = Scraper::new(
        String::from("scmp"),
        String::from("https://www.scmp.com/news/china/politics/"),
        vec![scraper::Selector::parse("#Content").unwrap()],
        vec![
            scraper::Selector::parse(".eg5341w15").unwrap(),
            scraper::Selector::parse(".article__link").unwrap(),
        ],
        crate::scanners::scanner::Country::China,
        None,
        None,
    );
    println!("Scraping scmp");
    scraper.scan().await;
    scraper
}
