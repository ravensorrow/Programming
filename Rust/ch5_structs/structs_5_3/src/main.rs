// Example 1: Converting functions to methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} spare pixles.",
        rect.area()
    );
}

// Example 2: more metod conversion
struct Rectangle1 {
    width: u32,
    height: u32,
}

impl Rectangle1 {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main1() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has the nonzero width; it is {}", rect1.width);
    }
}

// Example 3: Metonds with MORE parameters!
fn main2() {
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect3.can_hold(&rect4));
}
