use std::collections::HashMap;
use std::iter::Map;

use scraper::html::Select;
use scraper::selector::{self, Parser};

use crate::scanners::scanner::{self, Article};

use crate::client::get_stealh_client;

use super::scanner::Country;

pub struct Scraper {
    last_article: Option<scanner::Article>,
    articles: Vec<scanner::Article>,
    pub url: String,
    pub name: String,
    pub paragraphes_selectors: Vec<scraper::Selector>,
    pub news_selector: Vec<scraper::Selector>,
    pub country: scanner::Country,
    pub url_append: Option<String>,
    pub special_flag: Option<String>,
}

impl scanner::ScannerTrait for Scraper {
    fn get_country(&self) -> Country {
        self.country.clone()
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
            println!("error {}", response.unwrap_err());
            return None;
        }
        let response = response.unwrap();
        let document = scraper::Html::parse_document(&response);
        let content_selector: Vec<scraper::Selector> = self.paragraphes_selectors.clone();
        let mut all_paragraphs = 0;
        let mut content = String::new();
        for selector in content_selector {
            for x in document.select(&selector) {
                content.push_str(x.text().collect::<String>().as_str());
                all_paragraphs += 1;
            }
        }

        Some(scanner::ArticleBody { content })
    }

    fn get_articles(&self) -> Vec<scanner::Article> {
        self.articles.to_vec()
    }

    async fn scan(&mut self) -> Vec<scanner::Article> {
        let client = get_stealh_client();
        let response = client.get(&self.url).send().await;
        if response.is_err() {
            return vec![];
        }
        let response = response.unwrap();
        println!("request status {}", response.status());
        let response = response.text().await.unwrap();
        let document = scraper::Html::parse_document(&response);
        let mut titles: Vec<String> = Vec::new();
        let mut urls: Vec<String> = Vec::new();

        let content_selector: Vec<scraper::Selector> = self.news_selector.clone();
        for selector in content_selector {
            for x in document.select(&selector) {
                titles.push(x.text().collect::<String>());
                urls.push(x.value().attr("href").unwrap().to_string());
            }
        }
        let mut article_urls: HashMap<String, String> = HashMap::new();
        for (x, y) in titles.iter().zip(urls.iter()) {
            article_urls.insert(x.to_string(), y.to_string());
        }

        if let Some(x) = &self.special_flag {
            if x == "chinadaily" {
                for (x, y) in article_urls.clone() {
                    if !&y.contains("//www.chinadaily.com.cn/a/") {
                        article_urls.remove(&x);
                    }
                }
            }
        }

        for (i, mut j) in article_urls {
            if self.url_append.is_some() {
                j = format!("{}{}", self.url_append.clone().unwrap(), j);
            }

            let url = format!("{}", j);
            let mut article: Article = Article::new();

            article.set_name(&i);
            article.set_url(&url);

            if article.already_exists() {
                println!("{} already exists", i);
                continue;
            }

            let content = self.get_content_article(&url).await;

            match content {
                Some(x) => article.set_content(&x.content),
                None => continue,
            }

            article.save();

            self.articles.push(article.clone());
            self.last_article = Some(article.clone());
        }
        vec![]
    }
}

impl Scraper {
    pub fn new(
        name: String,
        url: String,
        paragraphes_selectors: Vec<scraper::Selector>,
        news_selector: Vec<scraper::Selector>,
        country: scanner::Country,
        url_append: Option<String>,
        special_flag: Option<String>,
    ) -> Self {
        Self {
            last_article: None,
            articles: vec![],
            url,
            name,
            paragraphes_selectors,
            news_selector,
            country,
            url_append,
            special_flag,
        }
    }
}
