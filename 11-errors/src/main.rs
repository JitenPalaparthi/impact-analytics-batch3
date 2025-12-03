use std::fs::File;

fn main() {
    let result: Result<File, std::io::Error> = File::open("logh.txt");

    match result {
        Ok(f) => {
            println!("File is opened")
        }
        Err(e) => {
            println!("there seems to be some error:{}", e);
        }
    }

    let result = divid_by_zero1(100, 2);
    match result {
        Ok(d) => {
            println!("div:{}",d)
        }
        Err(e) => {
            println!("There seems to be some error{}",e);
        }
    }

    let r =divid_by_zero2(10, 10);

    let r = divid_by_zero1(12, 4).expect("some result"); // very dangerouns, if no value , it panics

    println!("result:{}",r);

    let num : Option<i32> = Some(100);

    let v = num.unwrap(); // very dangerouns, if no value , it panics

    println!("{}",v *v);
   
}

// Result<T,E>
// Result<i32,String>
// Ok(i32)
// Err(String)

fn divid_by_zero1(num: i32, div: i32) -> Result<i32, String> {
    if div == 0 {
        Err("Divide by zero error".to_string())
        // Err("Divide by zero error".to_string())
    } else {
      Ok(num / div)
    }
}

fn divid_by_zero2(num: i32, div: i32) ->i32{
    if div == 0 {
       panic!("divide by zero is not allowed");
    }
    num/div
}

// Error handling

// recovarable errors
// errors are just values , just treated as strings
// most probably Result is treated as a ideamatic error handling in rust
// Result --> Enum
// Ok(T)
// Err(E)

// unrecovarable errors
// panic is a unrecovarable error
    //  system panics
    //  userdefined panic
