fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    // function pointer
    let c1 = calc1(Operations::Add);
    let r1 = c1(12, 23);
    println!("result: {}", r1);

    // impl pointer
    let c1 = calc2(Operations::Div);
    let r1 = c1(12, 23);
    println!("result: {}", r1);

    let c1 = calc3(Operations::Mul);
    let r1 = c1(12, 23);
    println!("result: {}", r1);
}

enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}
//

fn calc1(ops: Operations) -> fn(i32, i32) -> i32 {
    match ops {
        Operations::Add => add,
        Operations::Sub => sub,
        Operations::Mul => mul,
        Operations::Div => div,
    }
}

// impl block is static dispatch
// cheap allocated on stack
// compile time decsion
fn calc2(ops: Operations) -> impl Fn(i32, i32) -> i32 {
    match ops {
        Operations::Add => add,
        Operations::Sub => sub,
        Operations::Mul => mul,
        Operations::Div => div,
    }
}

// dyn block is static dispatch
// returned funcion is returned at time.
// since it returned at run time, how much memory to be given to the function sf is not known
// it has to wrapped with box, so that the memore is alloread at runtime on heap
fn calc3(ops: Operations) -> Box<dyn Fn(i32, i32) -> i32> {
    match ops {
        Operations::Add => Box::new(add),
        Operations::Sub => Box::new(sub),
        Operations::Mul => Box::new(mul),
        Operations::Div => Box::new(div),
    }
}

// fn calc4(ops: bool) -> Box<dyn Fn(i32, i32) -> i32> {
//     if ops {
//         return Box::new(add);
//     } else {
//         return Box::new(sub);
//     }
// }

// fn calc6(ops: bool) -> fn(i32, i32) -> i32 {
//     if ops {
//         return add;
//     } else {
//         return sub;
//     }
// }

// // not possible using impl block bxz which Fn to return is decidec at runtime.
// fn calc5(ops: bool) -> impl Fn(i32, i32) -> i32{
//     if ops {
//         return add;
//     } else {
//         return sub;
//     }
// }

// fn calc4(ops:Operations)->dyn Fn(i32,i32)->i32{
//     match ops{
//         Operations::Add=>add,
//         Operations::Sub=>sub,
//         Operations::Mul=>mul,
//         Operations::Div=>div,
//     }
// }


// fn vs impl Fn vs dyn Fn vs Box<dyn Fn>
