pub trait Fizzy{
    fn fizzy(&self) -> String;
}

impl Fizzy for i32 {
    fn fizzy(&self) -> String{
        match (self % 3, self % 5){
            (0, 0) => String::from("FizzBuzz!"),
            (0, _) => String::from("Fizz!"),
            (_, 0) => String::from("Buzz!"),
            _ => format!("{}", self)
        }
    }
}

fn main() {
    for x in 1..=100 {
        println!("{}", x.fizzy())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz() {
        for x in &[3, 6, 27] {
            assert_eq!(x.fizzy(), "Fizz!")
        }
    }

    #[test]
    fn test_buzz() {
        for x in &[5, 10, 20] {
            assert_eq!(x.fizzy(), "Buzz!");
        }
    }

    #[test]
    fn test_fizzbuzz() {
        for x in &[15, 30, 60] {
            assert_eq!(x.fizzy(), "FizzBuzz!");
        }
    }

    #[test]
    fn test_num() {
        for x in &[13, 29, 98] {
            assert_eq!(x.fizzy(), format!("{}", x));
        }
    }
}
