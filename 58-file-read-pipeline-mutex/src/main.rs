use std::{fs::File, io::Read};
use std::thread;
use std::io::{BufRead,BufReader};
use std::collections::VecDeque;
use std::sync::Condvar;

fn main() {
    let handler = thread::spawn(||{
        match File::open("data.txt") {
            Ok(mut file) => {
                let reader= BufReader::new(file);
                for line in reader.lines(){
                    let l = line.expect("line is not formed");
                    println!("{}",l);
                }
            }
            Err(e) => panic!("err opening file"),
        }
    });
}

// first , do the file read ,
// iterate thru the file
