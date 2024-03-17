use tiger::Tiger;
use tiger::Digest;

use std::{fs, io::Write};

#[derive(Debug, Clone)]
pub struct ArticleBody {
    pub content: String,
}

impl ArticleBody {
    pub fn new(content: String) -> ArticleBody {
        ArticleBody { content }
    }

    pub fn hash(&self) -> String {
        let mut hasher = Tiger::new();
        hasher.update(format!("{}", self.content));
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        hash
    }

    pub fn get_content(&self) -> String {
        self.content.clone().replace("\n", " ")
    }

    pub fn length(&self) -> usize {
        self.content.len()
    }
}

#[derive(Debug, Clone)]
pub struct Article {
    pub name: String,
    pub url: String,
    pub time: String,
    pub content: ArticleBody,
    pub interesting: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Country {
    China,
    Russia,
    French,
    None,
}

pub struct Scanner {
    country: Country,
    last_article: Option<Article>,
    articles: Vec<Article>,
}

pub trait ScannerTrait {
    fn get_country(&self) -> Country;
    fn get_last_article(&mut self) -> Option<Article>;
    fn get_articles(&self) -> Vec<Article>;
    async fn get_content_article(&mut self, url: &str) -> Option<ArticleBody>;
    async fn scan(&mut self) -> Vec<Article>;
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            country: Country::None,
            last_article: None,
            articles: Vec::new(),
        }
    }
}

impl Article {
    fn hash(&self) -> String {
        let mut hasher = Tiger::new();
        hasher.update(format!("{}{}{}", self.url, self.name, self.content.content));
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        hash
    }

    fn zip(&self) -> String {
        let mut hasher = Tiger::new();
        hasher.update(format!("{}{}{}", self.url, self.name, self.content.content));
        let hash = hasher.finalize();
        let hash = format!("{:x}", hash);
        hash
    }

    pub fn save(&self) -> () {
        if self.content.length() == 0 {
            error!("Empty article");
            return;
        }
        let str_final = self.hash();
        let content = format!(
            "url:{}\nname:{}\ncontent:{}",
            self.url, self.name, self.content.get_content()
        );
        fs::write(format!("./articles/{}", str_final), &content).unwrap();
    }

    pub fn already_exists(&self) -> bool {
        let str_final = self.hash();
        fs::metadata(format!("./articles/{}", str_final)).is_ok()
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.to_string();
    }

    pub fn set_url(&mut self, url: &String) {
        self.url = url.to_string();
    }

    pub fn set_content(&mut self, content: &String) {
        self.content.content = content.to_string();
    }
    pub fn new() -> Article {
        Article {
            name: String::from(""),
            url: String::from(""),
            time: String::from(""),
            content: ArticleBody {
                content: String::from(""),
            },
            interesting: None,
        }
    }
}
