use crate::scanners::scanner;

const URL: &str = "https://www.leparisien.fr/politique/";

use crate::client::get_stealh_client;

pub struct Leparisien {
    last_article: Option<scanner::Article>,
    articles: Vec<scanner::Article>,
}

impl scanner::ScannerTrait for Leparisien {
    fn get_country(&self) -> scanner::Country {
        scanner::Country::French
    }

    fn get_last_article(&mut self) -> Option<scanner::Article> {
        if self.last_article.is_none() {
            if !self.articles.is_empty() {
                self.last_article = Some(self.articles[0].clone());
            }
        }
        self.last_article.clone()
    }

    async fn get_content_article(&mut self, url: &str) -> Option<scanner::ArticleBody> {
        let client = get_stealh_client();
        let response = client.get(url).send().await.unwrap().text().await;
        if response.is_err() {
            return None;
        }
        let response = response.unwrap();
        let document = scraper::Html::parse_document(&response);
        let content_selector: Vec<scraper::Selector> = vec![
            scraper::Selector::parse(".article-section").unwrap(),
            scraper::Selector::parse(".margin_bottom_subheadline").unwrap(),
        ];
        let mut all_paragraphs = 0;
        let mut content = String::new();
        for selector in content_selector {
            for x in document.select(&selector) {
                content.push_str(x.text().collect::<String>().as_str());
                all_paragraphs += 1;
            }
        }

        println!("{} paragraphs", all_paragraphs);

        Some(scanner::ArticleBody { content })
    }

    fn get_articles(&self) -> Vec<scanner::Article> {
        self.articles.to_vec()
    }

    async fn scan(&mut self) -> Vec<scanner::Article> {
        let client = get_stealh_client();
        let response = client.get(URL).send().await.unwrap().text().await.unwrap();
        let document = scraper::Html::parse_document(&response);
        let title_selector = scraper::Selector::parse(".lp-card-article__link").unwrap();
        let titles = document
            .select(&title_selector)
            .map(|x| x.text().collect::<String>());
        let urls = document
            .select(&title_selector)
            .map(|x| x.value().attr("href").unwrap().to_string());

        for (i, mut j) in titles.zip(urls) {
            println!("{} {}", i, j);
            j = j[2..].to_string();
            let url = format!("https://{}", j);
            let content = self.get_content_article(&url).await.unwrap();

            self.articles.push(scanner::Article {
                name: i,
                url,
                time: String::from(""),
                content,
                interesting: None,
            });
        }
        vec![]
    }
}

impl Leparisien {
    pub fn new() -> Self {
        Self {
            last_article: None,
            articles: vec![],
        }
    }
}
