use std::ops::MulAssign;

fn main() {
  
  let arr1= [10,34,54,123,45,56,765,65];

  let max:i32= max_slice(&arr1[..]);

  let vec1= vec![10,34,54,123,45,56,765,65];

  let max:i32= max_slice(&vec1);

 let vec2: Vec<f64>= vec![10.3,34.4,54.5,123.6,45.7,56.8,765.9,65.10];

  let max:f64= max_slice::<f64>(&vec2);
}

// & [T]
fn max_slice<T>(arr:&[T])->T where T:PartialOrd+Copy+MulAssign {
    let mut max: T = arr[0];
    for i in arr{
        if *i > max{
            max = *i;
        }
    }
max
}

