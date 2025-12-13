use core::time;

use tokio::fs;
use tokio::io;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);
    //std::sync::mpsc::sync_channel(bound) // unbound

   let producer= tokio::spawn(async {
        let file: fs::File = fs::File::open("about.txt").await.unwrap();
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await.unwrap() {
             tx.send(line).await.unwrap();
             println!("Sending data");
        }
        drop(tx); // close 
    });


    let consumer= tokio::spawn(async move {
           while let Some(v)= rx.recv().await{
            tokio::time::sleep(time::Duration::from_millis(200)).await;
                println!("{v} len: {}",v.len());
           }
    });


      let r = tokio::join!(producer,consumer);

}

// mpsc
// oneshot

// broadcast
// watch
