use std::{rc::Rc, thread, time::Duration};
fn main() {
     let s1: mystring = mystring("Hello World".to_string());
        let s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());

        let s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
        let mut s1: mystring = mystring("Hello Impact Analyticsd".to_string());
    {
        let s1: mystring = mystring("Hello World".to_string());
        let s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());

        let s1: mystring = mystring("Hello World".to_string());
        let s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
        let mut s1: mystring = mystring("Hello World".to_string());
    }
    loop {}     // this would never end .. and any objects those are created before that not within a separate 
    // scope many not be dropped..
}
// listen and serving

#[derive(Debug, Clone)]
struct mystring(String);

impl Drop for mystring {
    fn drop(&mut self) {
        println!("Dropping mystring-->{} ", self.0)
    }
}
