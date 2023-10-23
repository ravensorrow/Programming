fn main() {
    {                       // 's' is not valid here, it's not yet decalred
        let s = "Hello";    // 's' is valid from this point forward
        //
        // do some stuff with 's'
    }                       //this scope is now over, and 's' is no lonher valid
    println!(s);
}
