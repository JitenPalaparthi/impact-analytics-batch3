use tokio::time;
use tokio::task::spawn_blocking;
#[tokio::main]
async fn main() {
 
 let task1=  async  {
    let mut sum = 0;
    for i in 1..10{
        tokio::time::sleep(time::Duration::from_millis(100)).await;
        println!("task-1 {}",i);
        sum +=1;
    }
    return sum;
 };

  let task2=  async  {
    for i in 10..20{
         tokio::time::sleep(time::Duration::from_millis(100)).await;
        println!("task-2 {}",i);
    }
 };

let task3: tokio::task::JoinHandle<i32>= tokio::spawn(async{
let mut sum = 0;
    for i in 1..10{
        tokio::time::sleep(time::Duration::from_millis(100));
        println!("task-3 {}",i);
        sum *=1;
    }
    return sum;

 });


 // FnOnce
 let task4= tokio::task::spawn_blocking(||{
let mut sum = 0;
    for i in 1..10{
        tokio::time::sleep(time::Duration::from_millis(100));
        println!("task-4 {}",i);
        sum *=1;
        sum+= 10 /2 + 10+20
    }
    return sum;

 });
 
 
// task1.await;
// task2.await;
let (r1,r2,r3,r4)=tokio::join!(task1,task2,task3,task4);

println!("{:?} {:?}",r1,r2);

}
