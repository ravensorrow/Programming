fn main() {
    /*
     * Variables are named locations in memory
     * Varliables are immutable by default
     */
    let mut value  = 9;
    println!("The value is {}", value);
    value = 10;
    println!("The value is {}", value);

    let small_value:i8 = 100;
    let sample_float:f32 = 700.25;
    println!("Small int {}", small_value);
    println!("Sample float {}", sample_float);

    let x = 64;
    println!("The value of x is {}", x);
    let x = x + 1; /* Shadowing */
    println!("The value of x is {}", x);
    let x = "RUST PROGRAMMING"; /* Shadowing and changing datatype of variable */
    println!("The value of x is {}", x);
    
    /* Arrays */
    let numbers = [1,2,3,4,5];
    println!("Element at index 0: {}", numbers[0]);

    /* Tuples */
    let person = ("Alice", 30, 5.4);
    println!("Name:{}", person.0);
    println!("Age:{}", person.1);
    println!("Height:{}", person.2);

    /* Constants */
    const PI:f32 = 3.14;
    println!("The Value of PI is {}", PI);

    /* Constants vs Immutable valiables */
    // const PI:f32 = 6.14;

}
