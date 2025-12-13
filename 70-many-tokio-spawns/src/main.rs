use core::time;

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {

    let mut handlers = Vec::new();

   let main_task = async{

    for i in 1..= 1000000{
        let handler = tokio::spawn(async move{
            for j in 0..=i{
                tokio::time::sleep(time::Duration::from_millis(500)).await;
                println!("i-->{} j:{}",i,j);
            }
        });
        handlers.push(handler);
    }
    for h in handlers{
        h.await.unwrap();
    }
   };

   tokio::join!(main_task);
    
}
