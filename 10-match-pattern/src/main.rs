fn main() {
    let str1: Option<String> = Some("Hello World".to_string());

    // if let Some(s) = str1 {
    //     // is str1 has some value
    //     println!("length of string:{}", s.len());
    // } else {
    //     println!("length of string:0");
    // }

    match str1 {
        None => println!("length of string:0"),
        Some(s) => println!("length of string:{}", s.len()),
    }

    let day = 4;

    match day {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("noday"),
    }

    let num = 2;

    match num {
        0 | 2 | 4 | 6 | 8 | 10 => println!("number is even number"),
        1 | 3 | 5 | 7 | 9 => println!("number is even number"),
        _ => println!("number is bigger than 10"),
    }

    let num = 2;
    match num {
        1..=10 => {
            println!("{} is between 1-10", num);
        }
        11..=20 => {
            println!("{} is between 11-20", num);
        }
        _ => println!("number is bigger than 20"),
    }

    let e = Direction::East;

    let mut w = Direction::West;

    w = Direction::South;

    match w {
        Direction::East => println!("East"),
        Direction::West => println!("West"),
        Direction::South => println!("South"),
        Direction::North => println!("North"),
    }

    let mut message = Message::None;

    message = Message::Text("Hello World".to_string());

    message = Message::Msg {
        to: 101,
        from: 777,
        msg: "Hello how are you doing".to_string(),
    };

    match message {
        
        Message::None => {
            println!("No message since none");
        }
        Message::Text(t) => println!("Sending message ---> {}", t),

        Message::Msg { to, from, msg } => {
            println!(
                "Sending message from {} to {} the message -->{}",
                from, to, msg
            );
        }
    }

    match calc(Some(100), Some(200)) {
        Some(r) => {
            println!("Result:{}", r);
        }
        None => {
            println!("None")
        }
    }

    let num = 100;

    match num {
        _ if num % 8 == 0 => {
            println!("number is divisible by 8 {}", num)
        }

        _ if num % 4 == 0 => {
            println!("number is divisible by 4 {}", num)
        }

        _ if num % 2 == 0 => {
            println!("number is divisible by 2 {}", num)
        }
        _ => {}
    }
}

// variables
// Enums

enum Direction {
    East,
    West,
    South,
    North,
}
enum Message {
    None,
    Text(String),                            // Unit struct, or Tuple Struct
    Msg { to: i32, from: i32, msg: String }, // struct
}

fn calc(a: Option<i32>, b: Option<i32>) -> Option<i64> {
    match a {
        Some(a1) => match b {
            Some(b1) => {
                return Some((a1 + b1) as i64);
            }
            None => return None,
        },
        None => return None,
    }
}
