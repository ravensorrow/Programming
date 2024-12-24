use num_traits::{ identities::Zero, PrimInt }; /* 0.2.14 */

pub trait Fizzy {
    fn fizzy(&self) -> String;
}

impl<T> Fizzy T
where
    T: PrimInt + Zero,
    T: Copy + Clone,
    T: std::fmt::Display,
{
    fn fizzy(&self) -> String {
        let zero = T::zero();
        let three = T::from(3).unwrap();    /* Never fails! */
        let five = T::from(5).unwrap();
        match (*self % three, *self % five) {
            (x, y) if x == zero && y == zero => String::from("FizBuzz!"),
            (x, _) if x == zero => String::from("Fizz!"),
            (_, x) if x == zero => String::from("Buzz!"),
            _ => format!("{}", self),
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


