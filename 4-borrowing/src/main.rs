fn main() {
    let s1 = "hello World".to_string();
    let s2 = s1; // ownership transfer
    let s3 = &s2; // Borrowing , instead of transferring the ownership you borrow
    println!("s2:{}",s2);
    let s4 = &s2;
    println!("s2:{}",s2);
    let s4 = &s2;

    let l = get_length1(&s2);
    println!("l:{} s2:{}",l,s2);

    // for a varialbe there can any number of immutable borrows 
    // or 
    // only one mutable borrow 

}

fn get_length1(s:&String)->usize{
    return s.len() // uisize 
    // drop(s)
}


// take two variables
// create a function that takes two references of above variables
// and return a value which is addition of those variables.


// take two variables
// create a function that takes two references of above variables
// and return reference that contains the addtion of two varialbes