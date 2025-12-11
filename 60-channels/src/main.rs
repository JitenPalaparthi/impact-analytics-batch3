// CSP --> White paper

use core::time;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (snd, rec) = mpsc::channel();
    let snd2 = snd.clone();
    let sender1 = thread::spawn(move || {
        for i in 1..=100 {
            thread::sleep(time::Duration::from_millis(500)); /
            // computation 
            snd.send(i).unwrap();
        }
    });

    let sender2 = thread::spawn(move || {
        for i in 200..=300 {
            thread::sleep(time::Duration::from_millis(100));
            snd2.send(i).unwrap();
        }
    });

    let receiver = thread::spawn(move || {
        for r in rec {
            println!("{}", r);
        }
    });

    sender1.join().unwrap();
    sender2.join().unwrap();
    receiver.join().unwrap();
}
