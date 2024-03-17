extern crate pretty_env_logger;
#[macro_use]
extern crate log;
mod client;
mod print_utils;
mod scanners;

use std::sync::Arc;
use print_utils::print_info;
use tokio::sync::Mutex;


use scanners::{china::chinadaily, french::lemonde, russia::tass, scanner::ScannerTrait, scrapers::Scraper};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut scanners_list: Vec<Scraper> = Vec::new();
    //let scanners = Arc::new(Mutex::new(scanners_list));

    scanners_list.push(lemonde::scan().await);
    print_info("lemonde done");
    scanners_list.push(tass::scan().await);
    print_info("tass done");
    scanners_list.push(chinadaily::scan().await);
    print_info("chinadaily done");


    for scanner in scanners_list {
        for link in scanner.get_articles(){
            println!("{}", link.name);
        }
    }

}
