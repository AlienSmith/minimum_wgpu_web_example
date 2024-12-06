use my_macro::EnumToFunction;

#[derive(EnumToFunction)]
enum Operations {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

// Define functions corresponding to each enum variant
fn add(param0: i32, param1: i32) {
    println!("Add Result: {}", param0 + param1);
}

fn subtract(param0: i32, param1: i32) {
    println!("Subtract Result: {}", param0 - param1);
}

fn multiply(param0: i32, param1: i32) {
    println!("Multiply Result: {}", param0 * param1);
}

fn divide(param0: i32, param1: i32) {
    // Avoid divide by zero
    if param1 != 0 {
        println!("Divide Result: {}", param0 / param1);
    } else {
        println!("Cannot divide by zero");
    }
}

fn main() {
    let add_op = Operations::Add(10, 5);
    add_op.execute(); // Output: Add Result: 15
    
    let sub_op = Operations::Subtract(10, 5);
    sub_op.execute(); // Output: Subtract Result: 5

    let mul_op = Operations::Multiply(10, 5);
    mul_op.execute(); // Output: Multiply Result: 50

    let div_op = Operations::Divide(10, 5);
    div_op.execute(); // Output: Divide Result: 2
}