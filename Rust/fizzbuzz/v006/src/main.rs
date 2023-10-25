use num_traits::{identities::Zero, PrimInt}; // 0.2.14
use std::io::{Result, Write};

pub trait Fizzy {
    fn fizzy(&self, writer: &mut impl Write) -> Result<()>;
}

impl<T> Fizzy for T
where
    T: PrimInt + Zero,
    T: Copy + Clone,
    T: std::fmt::Display,
{
    fn fizzy(&self, writer: &mut impl Write) -> Result<()> {
        let zero = T::zero();
        let three = T::from(3).unwrap();    /* Never fails! */
        let five = T::from(5).unwrap();
        match (*self % three, *self % five) {
            (x, y) if x == zero && y == zero => writeln!(writer, "FizzBuzz!"),
            (x, _) if x == zero => writeln!(writer, "Fizz!"),
            (_, x) if x == zero => writeln!(writer, "Buzz!"),
            _ => writeln!(writer, "{}", self),
        }
    }
}

fn main() {
    let mut out = std::io::stdout();
    if let Err(error) = (1..=100).try_for_each(|x| x.fizzy(&mut out)) {
        println!("IO Error Writing to Stream: {}", error)
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


