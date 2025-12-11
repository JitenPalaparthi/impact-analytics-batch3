// CSP --> White paper

use core::time;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (snd, rec) = mpsc::channel();
    let snd2 = snd.clone();
    let sender1 = thread::spawn(move || {
        let mut i =0;
        loop{
            thread::sleep(time::Duration::from_millis(500)); /
            // computation 
            snd.send(i).unwrap();
            i+=1;
            if i ==20{
                drop(snd);
                break;
            }
        }
    });

  
    let receiver = thread::spawn(move || {
        for r in rec {
            println!("{}", r);
        }
    });

    sender1.join().unwrap();
    receiver.join().unwrap();
}
