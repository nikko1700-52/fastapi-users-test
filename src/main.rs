use mathutils::{add, sub, mul, div};

fn main() {
    println!("Math Utilities Demo");
    println!("===================");
    
    let a = 10;
    let b = 5;
    
    println!("a = {}, b = {}", a, b);
    println!("add(a, b) = {}", add(a, b));
    println!("sub(a, b) = {}", sub(a, b));
    println!("mul(a, b) = {}", mul(a, b));
    
    match div(a, b) {
        Ok(result) => println!("div(a, b) = {}", result),
        Err(e) => println!("div(a, b) error: {}", e),
    }
    
    // Test division by zero
    println!("\nTesting division by zero:");
    match div(a, 0) {
        Ok(result) => println!("div(a, 0) = {}", result),
        Err(e) => println!("div(a, 0) error: {}", e),
    }
}
