fn main() {
    /* String Slices (&str) */
    let greeting: &str = "Hello, world!";   /* Hardcoded and immutable */
    println!("{}", greeting);

    /* String literals */
    let name = "John";      /* Implicitly assigned the &str type */
    println!("{}", name);   /* If we don't use the variable, cargo will warn it's unused */

    /* Owned strings - mutable */
    let mut name = String::from("Zenva");
    name.push_str(" Acadamy");  /* We're modifying the name string here */
    println!("{}", name);
}