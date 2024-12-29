fn main() {
   /*   IF STATEMENT:
        If a number is divisiable by 5 and 3 print 'TriQunit'
        If a number is divisiable by 6 and 4 print 'HexaQuad'
        Otherwise print 'Just another number'
    */

    let number: i32 = 30;
    if number % 5 == 0 && number % 3 == 0 {
        println!("{} is a TriQunit", number);
    } else if number % 6 == 0 && number % 4 == 0 {
        println!("{} is a HexaQuad", number);
    } else {
        println!("{} is just another number", number);
    }

    /*
        IF LET STATEMENT:
        If is_weekend is true, assign "Go Hiking" to the variable 'activity'
        otherwise, assign "Go to work"
    */

    let is_weekend:bool = true;
    let activity: &str = if is_weekend {"Go Hiking!"} else {"Go to work!"};
    println!("{} is today's activity", activity);

    let is_weekend:bool = false;
    let activity: &str = if is_weekend {"Go Hiking!"} else {"Go to work!"};
    println!("{} is today's activity", activity);

    /*
        FOR LOOP
        Iterate over a collection like arrays
    */

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    for elem in arr {
        println!("{}", elem);
    }

    /*
        WHILE STATEMENT:
        Print a count down and when counter reaches zero, print "LIFT OFF!"!
    */

    let mut counter: i32 = 10;
    while counter > 0 {
        println!("Countdown :{}", counter);
        counter -= 1;
    }
    println!("LIFT OFF!");

    /*
    LOOP STATEMENT
    */

    let mut index: i32 = 1;
    loop {
        index += 1;
        println!("Index {}", index);

        if index == 100 {
            println!("Max index reached!");
            break;
        }
    }
}