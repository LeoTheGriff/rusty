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
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let mut tup = (500, 6.4, 1);
    tup = (500, 6.4, 2);
}
