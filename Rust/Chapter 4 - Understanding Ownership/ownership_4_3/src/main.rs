// Example 1 (SLICE!)
//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//    s.len()
//}

// Example 2 (SLICE! An improvment!?!)
//fn main() {
//
//    let mut s = String::from("Hello World!");
//    
//    let _word = first_word(&s);  // word will get the value 5
//    // I had to modify this to remove a compile-time error. RUST did not like
//    // 'word' as a variable, even though the documentation said it was correct.
//    // So I appended an underscore to the beginning (as suggested) to make it 
//    // compile without an errors.
//    
//    s.clear(); // This empties the String, making it equal to ""
//    
//    // word still hasd the value 5 here, but there's no more strig that
//    // we could meaningfully use the value 5 with. word is now totally invalid
//
//} // we out!

// Example 3 (rewriting ex1)
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Example 4 (does not compile!)
//fn main() {
//    let mut s = String::from("Hello World!");
//
//    let word = first_word(&s);
//                        // E0502: An immputable borrow happens here!
//    s.clear(); // error!
//    // E0502: A mutable borrow happens right here!
//
//    println!("the first word is: {}", word);
//                                    // E0502: Another immputable borrow!
//}
 
// Example 5 (onward and upward!)
fn main() {
    let my_string = String::from("Hello World!");

    // 'first_word' works on sliaces of 'String's, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 'first_word' also works on references to 'Strin's, which are equivalent
    // to whole slices of 'String's
    let word = first_word(&my_string);

    let my_string_literal = "Hello World!";

    // 'first_word' works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(&my_string_literal);
}
