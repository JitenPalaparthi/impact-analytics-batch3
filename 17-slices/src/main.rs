fn main() {
    let mut arr1: [i32; 9] = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let slice1: &[i32] = &arr1[0..4]; // borrowing // ptr and also len
    let slice2: &[i32] = &arr1[4..=8]; // &mut 
    let slice3: &[i32] = &arr1[4..]; // borrowing rules
    let slice4 = &arr1[..];

    // &[i32]-> {ptr,len}
    //arr1[0]= 100;
    println!("slice1 len:{}", slice1.len());
    {
        let slice4 = &mut arr1[0..];
        let r1 = get_sum(slice4);
        let r1 = get_double_sum(slice4);
    }

    println!("{:?}", arr1);

    //     for i in slice1{
    //         println!("{}",i);
    //     }

    //    let r1 = get_sum(slice1);
    //    let r1 = get_sum(slice2);
    //    let r1 = get_sum(slice3);

    // let str1: &str = "Helllo Wrold"; // ptr and len
}

// &str --> string slice --> ptr and lengh

// continous block of memory --> [............]
//

fn get_sum(arr: &[i32]) -> i32 {
    // functions or methods are written for slices, not for arrays
    let mut sum = 0;
    for i in arr {
        sum += *i;
    }
    sum
}

fn get_double_sum(arr: &mut [i32]) -> i32 {
    // functions or methods are written for slices, not for arrays
    let mut sum = 0;
    for i in arr {
        *i = *i * 2;
        sum += *i;
    }
    sum
}

// take an array 
// fill the array with default value 0
// let arr = [0;10];
// take a rand libray and fill the array with random numbers
// create a function to return max and min numbers from a array
// make sure that the function works for all lengths of array
// if a single function return a touple (i32,i32)