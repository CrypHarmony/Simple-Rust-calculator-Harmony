// A simple calculator using enums and pattern matching in Rust


// Define an enum to represent different operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
// We define a function to perform operations based on the enum variant
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a,b) => a- b,
        Operation::Multiply(a,b) => a *b,
        Operation::Divide(a,b) => a/ b,
    }
}
// In the main function , we create instances of the Operation enum for different operations and call the calculate function to get the the results.
fn main () {
    let a = 13;
    let b = 7;

    let add_result = Operation::Add(f64::from(a), f64::from(b));
    let sub_result = Operation::Subtract(f64::from(a), f64::from(b));
    let mul_result =  Operation::Multiply(f64::from(a), f64::from(b));
    let div_result = Operation::Divide(f64::from(a), f64::from(b));

    println!("{} + {} = {}", a,b, calculate(add_result));
    println!("{} - {} = {}", a, b, calculate(sub_result));
    println!("{} * {} = {}", a, b, calculate(mul_result));
    println!("{} / {} = {}", a,b, calculate(div_result));
}


// Key features of this code include:

// 1. Enums: An enum(short for "enumeration")is a way to define a type that  can be  multiple different things(variants), not just one fixed structure
// 2. Pattern Matching: The match statement allows us to handle different variants of the enum so that we can perform
// different operations based on the the type of operation we want to to perform.




    
