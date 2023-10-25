pub trait Fizzy {
    fn fizzy(&self) -> String;
}

impl Fizzy for i32 {
    fn fizzy(&self) -> String {
        match(self & 3, self & 5) {
            (0, 0) => String::from("FizzBuzz!"),
            (0, _) => String::from("Fizz!"),
            (_, 0) => String::from("Buzz!"),
            _ => format!("{}", self),
        }
    }
}

fn main() {
    for x in 1..=100 {
        println!("{}", x.fizzy());
    }
}
