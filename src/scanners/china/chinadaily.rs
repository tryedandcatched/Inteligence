use crate::scanners::{scanner::ScannerTrait, scrapers::Scraper};

pub async fn scan() -> Scraper {
    let mut scraper = Scraper::new(
        String::from("ChinaDaily"),
        String::from("https://www.chinadaily.com.cn/"),
        vec![scraper::Selector::parse("#Content").unwrap()],
        vec![
            scraper::Selector::parse("a").unwrap(),
        ],
        crate::scanners::scanner::Country::China,
        Some(String::from("https:")),
        Some(String::from("chinadaily")),
    );
    println!("Scraping chinadaily");
    scraper.scan().await;
    scraper
}
