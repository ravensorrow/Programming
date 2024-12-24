fn main() {
    // Declaring scalar vaiables
    let small_value: i8 = 100;          // 8-bit signed integer
    let sample_float: f32 = -700.25;    // 32-bit float
    println!("Small int: {}", small_value);
    println!("Sample float: {}", sample_float);

    // Compound variables - Arrays
    let numbers = [0, 1, 2, 3, 4, 5];
    println!("Element at index 0: ({})", numbers[0]);

    // Compound variables - Tuples
    let person = ("Allice", 30, 5.4);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);

    // Constants
    const PI: f32 = 3.14;
    println!("Value of pi: {}", PI);
}