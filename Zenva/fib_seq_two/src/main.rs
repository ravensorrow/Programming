fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut count: u64 = 0;

    let range: u64 = args[1].parse().unwrap();

    if range <= 0 {
        println!("Please enter a positive number greater than 0.");
    } else if range == 1 {
        println!("{}", args[1])
    } else {
        while count < range {
            println!("count {} - fib - {}", count+1, &a);
            let nth = a + b;
            a = b;
            b = nth;
            count += 1;
        }
    }
}