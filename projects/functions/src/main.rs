/// Documentation comments are denoted by three slashes.
/// They support Markdown notation.
/// You can use `cargo doc --open` to generate and view the documentation for your project.
fn five() -> i32 {
    // You can return early from a function by using the return keyword and specifying a value,
    // but most functions return the last expression implicitly.
    4 + 1
}

fn main() {
    println!("Hello, world!");

    let x = five();

    print_labelled_measurement(x, 'm');

    compare_conditionals(3);
    compare_conditionals(6);

}

// You must declare the type of each parameter in the function signature.
fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement value is: {value}{unit_label}");
}

fn compare_conditionals(number: i32) {
    
    println!("Comparing conditionals for number: {}", number);

    // if-else
    // Expressions must evaluate to a boolean value.
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // if-else if-else
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // if-let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.  
    // let number = if condition { 5 } else { "six" }; // ERROR: `if` and `else` have incompatible types
    // Therefore they must be the same type in this expression
    println!("The value of number is: {}", number);
}