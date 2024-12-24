fn main() {
    /* Declaring a variable in Rust */
    let var = 9;
    println!("The value is {}", var);

    /* Reassigning values to an already existing variable is NOT allowed
       //value = 10; /* This will cause an error and compilation will fail */
    */

    /* You can make a vaiable mutable by adding the 'mut' keyword */
    let mut value = 9;
    println!("The value is {}", value);
    value = 10; /* This is not allowed! */
    println!("The value is {}", value);

    /* Shadowing in Rust */
    let x = 64;
    let x = x + 1;  /* Shadowing in action! */
    println!("The value is {}", x);

    let x = "Rust Programming"; /* The previous datatype was thrown out and replaced with this string datatype */
    println!("The value of x is {}", x);
}
