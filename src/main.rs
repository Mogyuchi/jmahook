use dotenv::dotenv;
use std::time::Duration;

mod feed;
mod report;
mod webhook;

fn main() {
    dotenv().ok();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let task = async {
        loop {
            println!("checking...eqvol");
            crate::feed::eqvol::eqvol().await.unwrap();
            tokio::time::sleep(Duration::from_secs(30)).await;
            println!("checking...extra");
            crate::feed::extra::extra().await.unwrap();
            tokio::time::sleep(Duration::from_secs(30)).await;
            println!("checking...other");
            crate::feed::other::other().await.unwrap();
            tokio::time::sleep(Duration::from_secs(30)).await;
            println!("checking...regular");
            crate::feed::regular::regular().await.unwrap();
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    };
    rt.block_on(task);
}
