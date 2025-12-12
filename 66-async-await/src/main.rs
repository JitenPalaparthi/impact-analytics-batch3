use core::time;

use tokio::fs;


#[tokio::main]
async fn main() {
    let content = fs::read_to_string("data.txt").await.unwrap();
    println!("content:{}",content);
    for i in 1..10{
        tokio::time::sleep(time::Duration::from_millis(500)).await;
        println!("this is run after await");
    }
}

