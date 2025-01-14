fn main() {
    println!("Hello, world!");


    println!("Hello, data types!");

    // NUMERIC OPERATORS
    // Basic operators as expected
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;

    // Integer division truncates
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    
    // BOOLEAN OPERATORS
    let t = true;
    let f: bool = false; // with explicit type annotation

    // TEXT
    // Characters are single quoted and represent a Unicode Scalar Value
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Strings are double quoted and UTF-8 encoded
    let s = "hello";


    println!("Hello Compound Types!");

    // Tuples group a number of values with a variety of types into one compound type.
    // Tuples have a fixed length.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuples are mutable and can be accessed by destructuring or index
    let mut tup = (500, 6.4, 1);
    println!("The value of y is: {}", tup.2);
    tup = (500, 6.4, 2);
    println!("The value of y is: {}", tup.2); // Accessing tuple elements by index, starting at 0
    let (x, y, z) = tup;
    println!("The value of y is: {}", y); // Using pattern matching to destructure a tuple

}
