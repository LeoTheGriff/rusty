fn main() {
    println!("Hello, world!");

    print_labelled_measurement(5, 'm');

}

// You must declare the type of each parameter in the function signature.
fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement value is: {value}{unit_label}");
}
