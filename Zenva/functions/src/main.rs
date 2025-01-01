fn main() {
    let name = String::from("Alice");
    greet_user(name);
    let sum = calculate_sum(5, 10);
    println!("The sum is {}", sum);
}

fn calculate_sum(a:i32, b:i32) -> i32 {
    let sum = a + b;
    sum
}

fn greet_user(name:String) {
    println!("Hello {}, welcome to Rust Programming", name);
}