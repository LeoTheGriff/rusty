fn five() -> i32 {
    // You can return early from a function by using the return keyword and specifying a value,
    // but most functions return the last expression implicitly.
    4 + 1
}

fn main() {
    println!("Hello, world!");

    let x = five();

    print_labelled_measurement(x, 'm');
}

// You must declare the type of each parameter in the function signature.
fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement value is: {value}{unit_label}");
}
