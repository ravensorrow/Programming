fn main() {

    /* Example 1 */
    let s1 = String::from("hello"); // s1 is the owner of hello
    let s2 = s1.clone();            // s2 is a clone
    println!("{s2} world!");
    println!("{s1} world!");

    let i =9;
    let j = i;
    // Note that transfer of ownership happens only for complex data types
    println!("{} {}",j,i); /* also, how does this actually equal 9 9!?! */
    
    
    /* Example 2 */
     
    /* References */
    let name = String::from("Rob");
    print_greeting(&name);
    println!("{name}");

    let x= 5;
    let y = x;
    println!("x= {x} y = {y}");
    
}

fn print_greeting(name: &String) {
    println!("Welcome {name}");
}