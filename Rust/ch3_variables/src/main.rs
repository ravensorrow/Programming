use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The valiue of x is: {x}");

    let _y = 2.0;        // floating-point 64 bit
    let _z: f32 = 3.0;   // floating-point 32-bit

    {
        let x = x * 2;
        println!("The value of x in the innter scope is: {x}");
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("The total number of seconds in three hours is: {THREE_HOURS_IN_SECONDS}");
    }
    println!("Hello, World!");

    let x5 = five();
    println!("The value of x5 is: {x5}");

    let x_plus_one = plus_one(5);
    println!("The value of x plus one is: {x_plus_one}");

    break_dash();
        a_function();
    break_dash();
        numeric_operations();
    break_dash();
        boolean_types();
    break_dash();
        character_types();
    break_dash();
        tuple_types();
    break_dash();
        array_types();
    break_dash();
        another_function(5);
    break_dash();
        print_labled_measurement(5, 'h');
    break_dash();
        why();
    break_dash();
        control_flow();
    break_dash();
        //infinite_loop();          // cut the loops nuts off before it got crazy
    break_dash();
        smart_looping();
    break_dash();
        loopty_loop_break();
    break_dash();
        a_streaming_loop_here_a_streaming_loop_there();
    break_dash();
        wait_a_loopin_minute();
    break_dash();
        ahh_a_loop();
    break_dash();
        for_loop();
    break_dash();

}

fn break_dash() {
    println!("-------------------------");
}

fn numeric_operations() {
    let sum = 5 + 10;               // Addition
    let difference = 95.5 - 4.3;    // Subtraction
    let product = 4 * 30;           // Multiplication
    let quotient = 56.7 / 32.2;     // Division
    let truncated = -5 / 3;         // Division
    let remainder = 43 % 5;         // remainder
    println!("5 + 10 is {sum}");
    println!("95.5 - 4.3 is {difference}");
    println!("4 times 30 is {product}");
    println!("56.7 divided by 32.2 is {quotient}");
    println!("-5 divided by 3 is {truncated}");
    println!("The remainder of 43 % 5 is {remainder}");
}

fn boolean_types() {
    let _t = true;
    let _f: bool = false;            // with explicit type annotation
}

fn character_types() {
    let _c = 'd';
    let _d: char = 'D';              //with explicit type  annotation
    let _heart_eyed_cat = '😻';
}

fn tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup_two = (500, 6.4, 1);
    let (_j, _k, _l) = tup;
    println!("The value of k is {_k}");

    let i: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = i.0;
    println!("Five Hundred is: {five_hundred}");
    let six_point_four = i.1;
    println!("Six point four is: {six_point_four}");
    let one = i.2;
    println!("One is: {one}");
}

fn array_types() {
    let _q = [1, 2, 3, 4, 5];
    let _q = [3; 5];
/*
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                   "August", "September", "October", "November", "December"];
    /*
    * This array and print statment does not work as epxected yet,
    * and that is perfectly OK at this poinmt. I will come back to 
    * fix it at a later date
    */
    println!("My birth months is {months}: {months.[8]}");   
*/ 
    

    let w: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let _first = w[0];
    let _second = w[1];

    println!("Please enter an array indexd.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line ");

    let index: usize = index
        .trim()
        .parse()
        .expect("Inded entered was not a number ");

    let element = w[index];

    println!("The value of the element at index {index} is: {element}");
}

fn a_function() {
    println!("Hello, Universe!");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn why() {
    let yy = 66;
    println!("The result of yy is: {yy}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    let number_three = 3;
    if number_three < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number_four = 4;
    if number_four == 3 {
        println!("The number is equal to three");
    } else if number_four != 0 {
        println!("The number was something other than zero");
    } else {
        println!("The number is: {number_four}");
    }

    let number_six = 6;
    if number_six % 4 == 0 {
        println!("This number is divisisable by 4");
    } else if number_six % 3 == 0 {
        println!("This number is divisable by 3");
    } else if number_six % 2 == 0 {
        println!("This number is divisable by 2");
    } else {
        println!("This number is not diviasble by 4, 3, or 2");
    }

    let condition = true;
    let number_three = if condition { 5 } else { 6 };
    println!("The value of number is: {number_three}");
}

fn _infinite_loop() {
    loop { /* You MONSTER! */
        println!("⭕️");
    }   /* You're really not going to fix this!?! */
}

fn smart_looping() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loopty_loop_break() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
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

    println!("End count = {count}");
}

fn a_streaming_loop_here_a_streaming_loop_there() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFEOFF!!!");
}

fn wait_a_loopin_minute() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn ahh_a_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn for_loop() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFEOFF!!!");
}
