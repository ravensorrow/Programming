<<<<<<< HEAD
use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // CONSTANTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *3;
    print!("Three hours as seconds is: {THREE_HOURS_IN_SECONDS}");
    println!("");   // line break
    // spaces
    // let mut spaces = "    ";
    // spaces = spaces.len();

    // floating point math {
    let x = 2.0;        //f64
    let y: f32 = 3.0;   //f32
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("");   //line break
    
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");
    println!("");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");
    println!("");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");
    println!("");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;     // Results in -1
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");
    println!("");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");
    println!("");

    // Boolean
    let t = true;
    let f: bool = false;    // with explicit type annotation
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");
    println!("");

    // Character Type
    let c = 'z';
    let z: char = 'ℤ';  // With explisit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
    println!("");

    // Compound type - Tuple
    /* 
    * This method does not actually work
    let tup: (i32, f64, u8) = 500, 6.4, 1);
    println!("The value of i32, f64, u8 is: {i32}, {f64}, {u8}"); 
    */

    // This is a better tuple-extraction method
    let tup = (500, 6.4, 1);
    let (x, y ,z) = tup;
    println!("The value of x, y, and z is: {x}, {y}, {z}");
    println!("");

    // An alternative method of extracting tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
    println!("");

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let third = a[2];
    let fourth = a[3];
    let fifth = a[4];
    println!("The value of a.0 (first) is: {first}");
    println!("The value of a.1 (second) is: {second}");
    println!("The value of a.2 (third) is: {third}");
    println!("The value of a.3 (fourth) is: {fourth}");
    println!("The value of a.4 (fifth) is: {fifth}");
    println!("");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

=======
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
<<<<<<< HEAD:Rust/Chapter 3 - Common Programming Concepts/variables/src/main.rs
>>>>>>> 230214f2aa1505de4f9f7ac15d3bb9c5fb91b66e
=======
>>>>>>> 230214f (Added Variables and Mutability, Data Types and Functions)
>>>>>>> 0dba5fc (rebase-cleanup):Rust/variables/src/main.rs
