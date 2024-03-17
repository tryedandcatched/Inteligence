extern crate pretty_env_logger;
#[macro_use] extern crate log;
mod client;
mod scanners;

use scanners::{
    china::{
        self,
        chinadaily::{self, Chinadaily},
        scmp,
    }, french::lemonde, russia::tass, scanner::ScannerTrait
};

#[tokio::main]
async fn main() {
        pretty_env_logger::init();
    let lm = lemonde::scan().await;
    for x in lm.get_articles() {
        println!("{:?}", x);
        
    }
}
