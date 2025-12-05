fn main() {
   
   let arr1: [i32; 10] = [10,32,23,43,3,56,76,145,345,23];

   let max = max_of_g(&arr1[..]);

   let arr1: [f32; 10] = [10.2,32.32,23.54,43.1,3.54,56.98,76.56,145.34,345.2,23.2323];

   let max = max_of_g_n(&arr1[..]);

   let arr1 = [10.2f64,32.32f64,23.54f64,43.1f64,3.54f64];

   let max = max_of_g_n(&arr1[..]);

}


fn max_of(slice: &[i32]) -> i32 { // works only with i32
    let mut max = slice[0];
    for i in slice {
        if *i > max {
            max = *i;
        }
    }
    max
}

// &[i32] &[i64] &[f32]. &[bool]


// >= <= > < != 

fn max_of_g<T: std::ops::Mul<Output = T>+PartialOrd+Copy>(slice: &[T]) -> T {
    let mut max = slice[0];
    for i in slice {
        if *i > max {
            max = *i;
        }
    }
    max
}

fn max_of_g1<T>(slice: &[T]) -> T  
        where T: std::ops::Mul<Output = T>+PartialOrd+Copy {
    let mut max = slice[0];
    for i in slice {
        if *i > max {
            max = *i;
        }
    }
    max
}


fn max_of_g_n<T: Number>(slice: &[T]) -> T {
    let mut max = slice[0];
    for i in slice {
        if *i > max {
            max = *i;
        }
    }
    max
}

fn get_val<T>(t:T)->T{
t
}

fn myprintln<T>(t:T) where T:std::fmt::Display{
    println!("{}",t);
}


trait Number: std::ops::Add<Output = Self>+Sized+PartialOrd+Copy{}

impl Number for i32{}
impl Number for f32{}
impl Number for f64{}