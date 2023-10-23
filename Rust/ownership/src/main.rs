fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");             // push_str(); appends a literal to a String
    println!("{}", s);                  // This will print 'hello world!'

    // The code above works!
    // The following code below will fail
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
