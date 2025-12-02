fn main() {
    // there is no null or nil in pure rust

    let mut val: Option<i32> = Some(100);
    //val = None;

    if let Some(v) = val {
        // this works well with Option type
        println!("{}", v);
    } else {
        println!("none");
    }



      let mut message = Message::None;

    if let Message::Text(t) = message {
        println!("Sending message ---> {}", t)
    } else if let Message::Msg { to, from, msg } = message {
        println!(
            "Sending message from {} to {} the message -->{}",
            from, to, msg
        );
    } else {
        println!("No message since none");
    }

    // val = None;

    // if let None = val{
    //     println!("none")
    // }else{
    //     println!("some value")
    // }

    // Option --> enum

    val = None;

    let r = get_some_or_default(val, 0);

    println!("{}", r);

    // println!("{:?}", val.unwrap());

    let (age, gender) = (41u8, 'm');

    if age >= 18 && (gender == 'f' || gender == 'F') {
        println!("She is eligible for marraige")
    } else if age >= 21 && (gender == 'm' || gender == 'M') {
        println!("he is eligible for marriage")
    } else {
        println!("invalid data")
    }

    let result = if age >= 18 && (gender == 'f' || gender == 'F') {
        "eligible"
    } else if age >= 21 && (gender == 'm' || gender == 'M') {
        "eligible"
    } else {
        "not eligible"
    };
}

fn get_some_or_default(val: Option<i32>, dval: i32) -> i32 {
    if let Some(v) = val {
        return v;
    } else {
        return dval;
    }
}

enum Message {
    None,
    Text(String),                            // Unit struct, or Tuple Struct
    Msg { to: i32, from: i32, msg: String }, // struct
}
