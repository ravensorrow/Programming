fn main() {
    /* Variables Challenge: Part One */

    /* 1. Store a person's name */
    let name = "Grunk";
    println!("{}", name);

    /* 2. Use a variable to store the person's current age */
    let my_age = 45;          // 8-bit signed integer storing an age
    println!("{}", my_age);

    /* 3. Use a constant for number of years */
    const YEAR: f32 = 2024.0;
    println!("The current year is: {}", YEAR);
    
    /* 4. Calculate the person's age in the future */
    const FIFTY_YEARS: f32 = YEAR + 50.0;
    let new_age = my_age + 50;
    
    /* 5. Print the name and the calculated future age */
    println!("{} is curently {} and will be {} in fifty years and the year will be {}", name, my_age, new_age, FIFTY_YEARS);
}