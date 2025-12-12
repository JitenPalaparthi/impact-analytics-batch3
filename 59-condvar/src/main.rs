use core::time;
use std::sync::Arc;
use std::sync::{Condvar, Mutex};
use std::thread;
fn main() {
    let shared = Arc::new((
        Mutex::new(shared {
            counter: 0,
            has_value: false,
            done: false,
        }),
        Condvar::new(),
    ));
    let shared_producer = Arc::clone(&shared);
    let shared_consumer = Arc::clone(&shared);

    let produce_handler = thread::spawn(move || {
        let (lock, cvar) = &*shared_producer;

        for i in 1u64..=10 {
            let mut state: std::sync::MutexGuard<'_, shared> = lock.lock().unwrap();

            while state.has_value {
                state = cvar.wait(state).unwrap();//unlocking it 
            }

            state.counter = i;
            state.has_value = true;
            state.done = false;
            println!("Producer produces:{}", i);
            cvar.notify_one();
            drop(state); // it is to unlock 

            thread::sleep(time::Duration::from_millis(500));
        }
            let mut state: std::sync::MutexGuard<'_, shared> = lock.lock().unwrap();
           state.done=true;
           cvar.notify_all();
           println!("Producer done producing values");

    });

    let consumer_handler = thread::spawn(move || {
        let (lock, cvar) = &*shared_consumer;
        loop{
            let mut state: std::sync::MutexGuard<'_, shared> = lock.lock().unwrap();

            // wait until there is not value 
            while !state.has_value && !state.done {
                state = cvar.wait(state).unwrap();
            }

            if !state.has_value && state.done{
                println!("consumer read everything.. producer stopped");
                break;
            }

            let value = state.counter;
            state.has_value = false;
            println!("Consumer consumerd:{}",value);
            cvar.notify_one();
        }

    });

    produce_handler.join().unwrap();
    consumer_handler.join().unwrap();
}

// if it a mutex, multiple threas can access the data.. but
// how do they notify each other
struct shared {
    counter: u64,
    has_value: bool, //flag
    done: bool,      //flag
}

// mutex != atomic
// a+ = 1 --> converted as atomic
