use std::rc::Rc;
fn main() {
    let mut s1: mystring = mystring("Hello World".to_string());
    {
        if true {
            let s2 = s1.clone();
            println!("get char count:{}", s2.0.len());
            println!("{:#?}", s2); 

        }
        //
        println!("Has s2 dropped");
    }
    println!("{:#?}", s1);
    
}

#[derive(Debug, Clone)]
struct mystring(String);

impl Drop for mystring {
    fn drop(&mut self) {
        println!("Dropping mystring-->{} ", self.0)
    }
}
