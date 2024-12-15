use std::io;

fn main() {
   println!("To quit the program, type 'quit'");
    loop {
        println!("Type a positive number now!");
        let mut int = String::new();
        io::stdin( )
        .read_line(&mut int)
        .expect("FAILURE! LOOSER! YOU FUCKED UP! TRY AGAIN! OR DONT!");
        if int.trim() == "quit" {
            println!("FUCKING LOOSER! FUCKING QUITTER! NO ONE WANTS YOU!");
            break;
        }
        let int: u32 = match int.trim()
        .parse() {
            Ok(int) => int,
            Err(_) => continue,
        };
    println!("Fibonacci ({}) => {}", int, fib(int.try_into().unwrap()));
    }
}

fn fib (n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n== 1 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}
