use std::{fmt::Debug, io::empty};

fn main() {
   
   let g = ||{
    println!("Hello World!");
   };
   
   g();

   (||println!("Hello Impact Analytics")); // created, not stored in any varialbe but directly executed

   let add = |a:i32,b:i32|->i32{
    a+b
   };

   let r = add(10,20); // Copy traint has been implementd on i32
   println!("result:{}",r);

   let r= (|a:i32,b:i32|->i32{
    a+b
   })(100,200);

   println!("result:{}",r);

   let vec1: Vec<i32> = vec![123,32,54,76,75,45,67,89,345];

    let max_fn=|vet:&Vec<i32>|->i32{
        let mut max = vet[0];
        for v in vet{
            if *v >max{
                max= *v;
            }
        }
        max
    }; // This is Fn


    let max = max_fn(&vec1);

    println!("max:{}",max);


   let fn1= |i:&dyn empty,j:&dyn empty|->bool{
         println!("{:?} {:?}",i,j);
         false
     };

     let r =fn1(&"hello world".to_string(),&"Hello Impact analytics".to_string());

     println!("{}",r);

     let r =fn1(&true,&false);

     println!("{}",r);



// Fn --> trait , where there is a closure or anonymous function, or a return type of a function but , the input arguments 
// can only be immutable borrows

}

impl empty for i32{}
impl empty for i64{}
impl empty for f32{}
impl empty for f64{}

impl empty for String{}

impl empty for bool{}

trait empty:Debug{} // It has implemented nothing

// Fn -->Normal function with imutable borrow
// FnMut --> Mutable borrow
// FnOnce --> Ownership transfer

// fn --> function pointer


fn calc(a:i32,b:i32,fn1:fn(i:i32,j:i32)->i32)->i32{
    return fn1(a,b)
}