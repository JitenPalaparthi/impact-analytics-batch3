// CSP --> White paper

use core::time;
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    let (snd, rec) = mpsc::sync_channel(0);
    let sender1 = thread::spawn(move || {
        for i in 1..=100 { 
            // computation 
            let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
        println!("{}",now.as_micros());
            snd.send(i).unwrap();
        }
    });

 

    let receiver = thread::spawn(move || {
        for r in rec {
            thread::sleep(time::Duration::from_millis(500));
            println!("receiver is running--->");
            println!("{}", r);
        }
    });

    sender1.join().unwrap();
    receiver.join().unwrap();
}
