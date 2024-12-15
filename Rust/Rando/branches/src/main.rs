fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisable by 4");
    } else if number % 3 == 0 {
        println!("Number is divisable by 3");
    } else if number % 2 == 0 {
        println!("Number is divisable by 2");
    } else {
        println!("Number is not divisable by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
