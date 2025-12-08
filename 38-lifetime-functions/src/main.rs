fn main() {

    let str1 = "Hello World".to_string();
    let str2 ="Hello impact analytics".to_string();
    let b1 = Box::new(100);
     // b1 is drooped
     {
        let str1 = "Hello World".to_string();
        let str2 ="Hello impact analytics".to_string();
        // dropped
       // println!("{}",b1);
        // 
     }

    let l = get_len(&str1);

    let max = get_str_max(&str1, &str2);
    // dropped here str1 and str2

    println!("Helllo lifetimes");

    let mut str1 ="Hello".to_string();

    let mut str2="Hello WOrld".to_string();
    // drop(str1) does not do that

    let s1= get_max_str(&str1,&str2);
    // str1 
    println!("{}",s1);
    // all of them are dropped here
    
    // drop(str1) , drop(str2)

    // why cant rust drop everything here -->? 
}

// for the below three functions , there is no need of giving lifetime annotations
fn get_len<'a>(s: &'a str) -> usize { // 
    return s.len();
}

fn get_sq<'a>(n: &'a mut i32) -> i64 { //
    *n = (*n * *n);
    return *n as i64;
}

fn get_str_max<'a,'b>(s1: &'a str, s2: &'b str) -> usize {
    if s1.len() > s2.len() {
        return s1.len();
    }
    s2.len()
}

fn get_str_max2(s1: &str, s2: &str) -> usize {
    if s1.len() > s2.len() {
        return s1.len();
    }
    s2.len()
}


fn get_max_str<'a>(s1: &'a str, s2:&'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}

// fn get_max_str2() -> &'static str {
//     "Hello World"
// }




// Where lifeteims should be used? -> Where ever there are references
