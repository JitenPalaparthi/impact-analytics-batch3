use core::time;
use std::{fs::File, io::Read};
use std::thread;
use std::sync::mpsc;
use std::io::{BufRead,BufReader};
use std::collections::VecDeque;
use std::sync::{Condvar};

fn main() {
    let (snd1,rec1)= mpsc::sync_channel(5); // 
   

    let sender = thread::spawn(move ||{
        match File::open("data.txt") {
            Ok(mut file) => {
                let reader= BufReader::new(file);
                let mut line_no=1;
                for line in reader.lines(){
                    let l = line.expect("line is not formed");
                    let line_no_str = format!("{}-->{}",line_no,l);
                    snd1.send(line_no_str.clone()).unwrap();
                    // println!("{}",l);
                    line_no+=1;
                }
            }
            Err(e) => panic!("err opening file"),
        }
        drop(snd1);
    });


    let receiver =thread::spawn(move||{
        for l in rec1{
            thread::sleep(time::Duration::from_millis(200)); 
            println!("rec-1-->{}",l);
        }
    });

    sender.join().unwrap();
    receiver.join().unwrap();
}

// first , do the file read ,
// iterate thru the file
