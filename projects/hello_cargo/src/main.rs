fn main() {
    println!("Hello, world!");


    println!("Hello, data types!");
    // NB: Variables are prefixed _ to suppress warnings about unused variables

    // NUMERIC OPERATORS
    // Basic operators as expected
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;

    // Integer division truncates
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;
    
    // BOOLEAN OPERATORS
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // TEXT
    // Characters are single quoted and represent a Unicode Scalar Value
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Strings are double quoted and UTF-8 encoded
    let _s = "hello";


    println!("Hello Compound Types!");

    // TUPLES
    // Tuples group a number of values with a variety of types into one compound type.
    // Tuples have a fixed length.
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuples are mutable and can be accessed by destructuring or index
    let mut _tup = (500, 6.4, 1);
    println!("The value of y is: {}", _tup.2);
    _tup = (500, 6.4, 2);
    println!("The value of y is: {}", _tup.2); // Accessing tuple elements by index, starting at 0
    let (_x, _y, _z) = _tup;
    println!("The value of y is: {}", _y); // Using pattern matching to destructure a tuple

    // The tuple without any values has a special name, unit.
    // Expressions implicitly return the unit value if they don’t return any other value.
    let _unit = ();
    let _unit: () = ();
    // return _unit;

    // ARRAYS
    // Arrays have a fixed length and contain elements of the same type.
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
    let a = [3; 5]; // [3, 3, 3, 3, 3] // Initial is the value;  Second is the length to repeat.

    print_array(a);

    // Accessing array elements
    // Arrays are zero-indexed and strictly typed.
    // Accessing an index that is out of bounds will cause a runtime error
    let _first = a[0];
    let _second = a[1];

    // VECTORS
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    let _v = vec![1, 2, 3, 4, 5]; // A vector is represented by Vec<T>
    let _v: Vec<i32> = Vec::new(); // with explicit type annotation


}

// Another function
fn print_array(a: [i32; 5]) {
    println!("The value of a is: {:?}", a);
}