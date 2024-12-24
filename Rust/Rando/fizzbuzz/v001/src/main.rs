fn main() {
    for x in 1..=100 {
        if x % 3 == 0 && x % 5 == 0 {   /* For every multiple of 3 and 5 
            println!("FizzBuzz")            print this! */
        } else if x % 3 == 0 {          /* For every multple of 3
            println!("Fizz")                print this! */
        } else if x % 5 == 0 {          /* For every multiple of 5
            println!("Buzz")                print this! */
        } else {
            println!("{}", x)           /* Print everything else from 1..100 */
        }
    }
}
