// Example 1 (works)
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
} // Here, 's' goes out of scope. But because it dose not have ownership of
  // what it refers to, it is not dropped.

// Example 2 (does not work)
//fn main2() {
//    let s = String::from("hello");
//    change(&s);
//}
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}


// Example 3 (example 2, corrected)
// While this code *works* in that it does not fail to compile. RUST will warm
// that it is dead code, because the syntax is tenchnically correct, The
// functions are never called, thus 'dead code'.
fn main3() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Example 4 (does not compile)
// Illegal borrow action! Can't borrow a mutable 's' because it's already 
// been loaned out.
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);

// Example 5 (correction on ex4) (code does not compile)
let mut s = String::from("hello");
{ let r1 = &mut s; }    // r1 goes out of scope here, so we can make a new 
                        // reference with no problems.
let r2 = &mut s;

// Example 6 (this is how you break shit!)
let mut s = String::from("hello");
let r1 = &s; // No problem!
let r2 = &s; // Still no problem!
let r3 = &mut s; // NOW YOU DUN FUCKED UP!
println!("{}, {}, and {}", r1, r2, r3);

// Example 6 (this code technically works if in a main() function, but will not
//              work here because of the way this document is organized)
let mut s = String::from("hello");
let r1 = &s; // Not the droids you're looking for!
let r2 = &s; // Still not the droids you're looking for!
println!("{} and {}", r1, r2);  // Valid as vars r1 and r2 will not be used after this
let r3 = &mut s;    // Also not the droids you are looking for!
println!("{}", r3);

// Tidbit of programming wisdom: a compile time bug is easier to track down
//                                  vs a runtime bug. RUST will tell you 
//                                  exactly where to start looking when the
//                                  compiler failes to compile. DUH!

// Example 7 (dangling references) (this code will not compile!)
fn main4() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

// Example 8 (introspection of example 7)
fn dangle() -> &String { //dangle reutns a reference to a stringify!
    let s = String::from("hello");  //s is a new String
    &s // we return a reference to the String, static 
}   // Here, s goers out of scope, and is dropped. It's memory goes away.
// DANGER WILL ROBINSON! DANGER!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// You can have at any given time one mutable reference or any number of 
// mutable references. BUT THEY HAVE TO BE USED! You cannot reuse a mutable
// reference, and you cannot use a mutable reference outside of it's scope (i.e.
// when you leave the scope of the function where it was created and first used.)

