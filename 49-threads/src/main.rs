use core::time;
use std::thread;

fn main() {

    let handle1: thread::JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("thread-1 {}-->", i);
            // thread::sleep(time::Duration::from_nanos(10));
            // thread::sleep(time::Duration::from_millis(500)); // you are blocking the thread
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..10 {
            println!("thread-2 {}-->", i);
            //thread::sleep(time::Duration::from_millis(500)); // you are blocking the thread
        }
    });
    let mut sum = 0;

    let handle3 = thread::spawn(move || -> i32 {
        
        for i in 1..=10 {
            sum += i;
        }
        return sum;
    });

    println!("main has ended");

    handle1.join().unwrap();
    handle2.join().unwrap();
    let sum= handle3.join().unwrap();
    println!("The sum of elements of thread-3: {}",sum);

}

// 16 core processor.
// can run 16 threads at any point of time.

// Multiplexing --> M:N
// M is the OS thread
// N is the hardware thread

// each process has 1 thread by default

// there is no hirarchy of threads.. according to OS

// os scheduler --> preemptive threads for ex every 10 milli seconds (subjective)
// each thread has stack that is 2mb (subjective to os setup or config)

// main thread does not wait for any other thread to complete its execution

// in multi thread application , it is not guaranteed to be exeucxuted in an order by design
//
