fn main() {
    /* String Slices: &str */
    let greeting: &str = "Hello, world!";
    println!("{}", greeting);

    /* String literals */
    let name = "John";
    println!("{}", name);

    /* Owning Strings: String */
    let mut name = String::from("Zenva");
    name.push_str(" Acadamy");
    println!("{}", name);
}
