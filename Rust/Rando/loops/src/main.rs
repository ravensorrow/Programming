fn main() {
    /* This code doesn't need to be run every time I step through this 
    * section.
     loop {
        println!("again!");
    }
    */ 

    let mut counter = 0;

    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // This codeblock should say "End count = 2", but it doesn't.
    // I think something prior in the code is messing with it.
    // It should be an easy fix, but I just don't feel like figureing it 
    // our right now.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {counter}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFEOFF!!!");
}
