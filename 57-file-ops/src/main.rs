use std::{fs::File, io::Read};

use std::thread;

fn main() {
    let handler1 = thread::spawn(|| -> String {
        match File::open("data.txt") {
            Ok(mut file) => {
                let mut file_data = String::new();
                match file.read_to_string(&mut file_data) {
                    Ok(size) => {
                        println!("File successfully read! --> file size-->{}", size);
                        file_data
                    }
                    Err(e) => {
                        println!("something went wrong: {}", e);
                        "".to_string()
                    }
                }
            }
            Err(e) => panic!("err opening file"),
        }
    });

    let handler2 = thread::spawn(|| -> Vec<String> {
        match File::open("data.txt") {
            Ok(mut file) => {
                let mut lines: Vec<String> = Vec::<String>::new();
                let mut line = "".to_string();
                loop {
                    let mut buf: [u8; 1] = [0; 1];
                    match file.read(&mut buf[..]) {
                        Ok(size) => {
                            if size == 0 {
                                println!("It has finished reading");
                                break lines;
                            } else {
                                if buf[0] == b'\n' {
                                    lines.push(line.clone());
                                    line.clear();
                                } else {
                                    line.push(buf[0] as char);
                                }
                            }
                        }
                        Err(e) => {
                            println!("It has finished reading-->{}", e);
                            break lines;
                        }
                    }
                }
            }
            Err(e) => panic!("err opening file"),
        }
    });

    let filedata = handler1.join().unwrap();
    println!("{}", filedata);
}

// first , do the file read ,
// iterate thru the file
