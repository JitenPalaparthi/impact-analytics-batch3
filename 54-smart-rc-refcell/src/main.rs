use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let data = Rc::new(RefCell::new(100));
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);

    *data1.borrow_mut() += 100;
    *data2.borrow_mut() += 100;

    println!("data:{}",data.borrow());

    let rc_str1 = Rc::new(RefCell::new("Hello World! ".to_string()));

    let rc_str2 = Rc::clone(&rc_str1);
    let rc_str3 = Rc::clone(&rc_str1);
    
    // let str1 = "hello Impact ANalytics".to_string();
    // let str2 = str1;
    {
        let mut s1 = rc_str2.borrow_mut();
        s1.clear();
    }
     println!("nothing here--> {}",rc_str1.borrow());
    {
       let mut s2 = rc_str3.borrow_mut();
        s2.push_str("Hello Impact Analytics");
    }

    println!("{}",rc_str1.borrow());

}
