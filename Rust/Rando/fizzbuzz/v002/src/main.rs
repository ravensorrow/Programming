fn main() {
    for x in 1..=100 { 
        match (x % 3, x % 5) {                  /* Setting up the pattern matching */
            (0, 0) => println!("Fizzbuzz!"),    /* Match multiples of both, do this */
            (0, _) => println!("Fizz!"),        /* Match multples of 3, do this */
            (_, 0) => println!("Buzz!"),        /* Match multples of 5, do this */
            _ => println!("{}", x),             /* Finish! Light a cig! */
        }
    }
}
