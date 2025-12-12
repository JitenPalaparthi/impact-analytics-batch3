use core::time;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::File, io::Read};

fn main() {
    let (snd, rec) = mpsc::channel();
    let rec: Arc<Mutex<mpsc::Receiver<String>>> = Arc::new(Mutex::new(rec));
    // let snd1 = snd.clone();
    // let snd2 = snd.clone();
   
    let sender = thread::spawn(move || {
        match File::open("data.txt") {
            Ok(mut file) => {
                let reader = BufReader::new(file);
                let mut line_no = 1;
                for line in reader.lines() {
                    let l = line.expect("line is not formed");
                    let line_no_str = format!("{}-->{}", line_no, l);
                    snd.send(line_no_str).unwrap();
                    println!("{}", l);
                    line_no += 1;
                }
            }
            Err(e) => panic!("err opening file"),
        }
        drop(snd);
    });

    let mut workers = Vec::new();

    // 10 workers

    for i in 1..=100{
        let rec1 = Arc::clone(&rec);
        let worker = thread::spawn(move || {
            loop {
                let msg = rec1.lock().unwrap().recv();
                match msg {
                    Ok(s) => {
                        println!("received by worker->{}-->{}", i, s);
                    }
                    Err(e) => {
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    println!("all workers stopped ");

    for w in workers {
        w.join().unwrap();
    }
    sender.join().unwrap();
}

// first , do the file read ,
// iterate thru the file
