use std::rc::Rc;
fn main() {
    let mut rc_str: Rc<String> = Rc::new("Hello World".to_string());
    println!("reference count:{}", Rc::strong_count(&rc_str));

    {
        let rc_str1 = Rc::clone(&rc_str); // increase the count of the rc
        println!("reference count:{}", Rc::strong_count(&rc_str));
    } // refcount -1

    {
        let rc_str2 = Rc::clone(&rc_str); // increase the count of the rc
        // drop
        println!("reference count:{}", Rc::strong_count(&rc_str));
    } // ref count=1

    println!("reference count:{}", Rc::strong_count(&rc_str));

    // Rc understand that the references are 1, so just deallocated the original memory
}

// Rc and Arc
// drop the only if the rc count is 0
