fn main() {
let v = 10;
 const NUM:i32= 60;
 const MIN:i32 = NUM * NUM ; 

 const LEN:usize=5;

 let arr1: [i32; LEN] = [10,20,30,40,50];

 let r =get_sum(arr1);

 let arr2: [i32; 6] = [10,20,30,40,50,60];

 let r =get_sum2(arr2);

 let arr2d: [[i32; 2]; 2] = [[1,2],[4,5]];
    // what is the type of array ?

let arr3d:[[[i32; 2]; 2]; 2]  = [[[1,2],[4,5]],[[6,7],[8,9]]];

let mut sum = 0;// there is not zero values, there is not value inference

for i in arr3d{

    for j in i{

        for k in j{
            sum+=k;
        }
    }
}
println!("Sum:{}",sum);

let arr1=[0;5];

println!("{:?}",arr1);

}

// Arrays --> are fixded length, length to be know to the compiler


fn get_sum(arr:[i32;5])->i32{
    let mut sum =0;
    for i in arr{
        sum+=i;
    }
    return sum;
}

fn get_sum2(arr:[i32;6])->i32{
    let mut sum =0;
    for i in arr{
        sum+=i;
    }
    return sum;
}


// @comptime for i in 1..=10{

// }

// comptime --> this would be evaluated at compile time
// LAZY for Global variables
// Comp timer semantics
