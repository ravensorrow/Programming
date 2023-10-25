use num_traits::{identities::Zero, PrimInt}; // 0.2.14
use std::fmt;

#[derive(Debug, PartialEq)]
enum FizzbuzzResult<T> {
    Fizz,
    Buzz,
    FizzBuzz,
    Num(T),
}

impl<T> fmt::Display for FizzbuzzResult<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FizzbuzzResult::Fizz => write!(f, "Fizz"),
            FizzbuzzResult::Buzz => write!(f, "Buzz"),
            FizzbuzzResult::FizzBuzz => write!(f, "FizzBuzz"),
            FizzbuzzResult::Num(ref val) => write!(f, "{}", val),
        }
    }
}

fn fizzbuzz<T>(num: T) -> FizzbuzzResult<T>
where
    T: PrimInt + Zero,
    T: Copy + Clone,
{
    let zero = T::zero();
    // These will never fail
    let three = T::from(3).expect("Could not convert '3' to generic type");
    let five = T::from(5).expect("Could not convert '5' to generic type");

    match (num % three, num % five) {
        (x, y) if x == zero && y == zero => FizzbuzzResult::FizzBuzz,
        (x, _) if x == zero => FizzbuzzResult::Fizz,
        (_, x) if x == zero => FizzbuzzResult::Buzz,
        _ => FizzbuzzResult::Num(num),
    }
}

fn main() {
    for x in 1..=100 {
        println!("{}", fizzbuzz(x))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz() {
        for x in &[3, 6, 27] {
            assert_eq!(x.fizzy(), "Fizz!");
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


