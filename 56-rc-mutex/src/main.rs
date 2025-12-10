use std::thread;
use std::sync::{Arc,Mutex};

#[allow(static_mut_refs)]
fn main() {

    let counter = Arc::new(Mutex::new(0));
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);
    
    let handler1= thread::spawn(move || {
        for i in 1..=100 {
            let result = counter1.lock();
            match result{
                Ok(mut v)=>{
                    *v+=1;
                }
                Err(e)=>println!("Some error:{:?}",e)
            }
            // // This would unlocked
            // println!("Hello world");

             let(a,b)    = (100,200);

            let c1: i32 = a+b * a-b;

        }
    });

    let handler2= thread::spawn(move || {
        for i in 1..=100 {
             let mut c = counter2.lock().unwrap();
            *c-=1;
        }
    });

    handler1.join().unwrap();
    handler2.join().unwrap();
    println!("Counter:{}",*counter.lock().unwrap());

}
