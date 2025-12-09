fn main() {
    let v2 = vec![10, 20, 30];

    let vec_mut_from_slice1 = |slice: &[i32]| -> i32 {
        let mut v1 = vec![10, 20];
        let mut sum = 0;
        for i in slice {
            v1.push(*i);
        }
        for v in &v1 {
            sum += v;
        }
        for v in &v2 {
            sum + v;
        }
        sum
    };

    let mut v1: Vec<i32> = vec![123, 32, 23, 23, 232];
    // let mut v2 = vec![10,20];
    // Explicitely there is no borrow or mutable borrow
    let mut vec_mut_from_slice2 = |slice: &[i32]| -> i32 {
        let mut sum = 0;
        for i in slice {
            v1.push(*i);
        }
        for v in &v1 {
            sum += v;
        }
        sum
    };

    let mut v1: Vec<i32> = vec![123, 32, 23, 23, 232];
    let mut v2 = vec![10, 20];
    let mut sum =  0; // copy trait has been imnplemented
    // Explicitely there is no borrow or mutable borrow
    let vec_mut_from_slice3 = move |slice: &[i32]| -> i32 {
        for i in slice {
            v1.push(*i);
        }
        for v in v1 {
            sum += v;
        }
        println!("sum inside closure:{}",sum);
        sum
    };

    let arr1 = [10, 20, 30, 40];

    let sum1 = vec_mut_from_slice1(&arr1[..]);
    println!("sum1 from Fn={}", sum1);

    let sum2 = vec_mut_from_slice2(&arr1[..]);
    println!("sum2 from FnMut={}", sum2);

    let sum3 = vec_mut_from_slice3(&arr1[..]);
    println!("sum3 from FnOnce={}", sum3);

    println!("{:?}", v2);
    // println!("{:?}",v1);
    println!("{:?}", sum);


    let mut num: i32 = 1; // it is a copy --> it is datatype
        num+=1;

        let vec1 = vec![10,20];// not implemented copyh traint

        // let num2 = num; // copy of the same data into a new variable even. move is used
        // let vec2 = vec1; // the ownership of data is moved into a separate scope.. the closure scope

         let mut fn1 = move ||->i32{
            num+=1;
            for v in vec1{
                num+=v;
            }
            num
         };


    // struct FnMut_Fn1<'a>{
    //     num:&'a mut i32,
    //     fn1:fn()->i32,
    // }

    // struct Fn_Fn1{
    //     num:i32,
    //     fn1:fn()->i32,
    // }


   // println!("{}",num);

   let  num2=  fn1(); 
   //println!("{} {} {:?}",num2,num,vec1); // 
     println!("{} {}",num2,num); // 

}

// Fn

// Fn --> FnMut --> FnOnce
