// Example 1: Basic format of a struct object in Rust.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Example 2: How to manipulate the struct
fn main() {
    let user1 = User {
        active: true,
        username: String::from("SomeUserName123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // LOOK MOM! I'M MUTABLE!
    user1.email = String::from("anotheremail@excaple.com");
}

// Example 3: Returning a User instance!
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Example  4: Shorthand implmementation of the above example
fn build_user1(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Example 5: new instances of User in user2
fn main1() {
    // --snip-- 
    // See main() for snipped functions
    // Depends on User struct.

    let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another.email@example.com"),
            sign_in_count: user1.sign_in_count,
    };
}

// Example 6: Same effect as above, but with less overall code.
fn main2() {
    // --snip--
    // See main() for snipped functions
    // Depends on User struct

    let user2 = User {
        email: String::from("yet.another.email@example.com"),
        ..user1
    };
}

// Example 7: tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main3() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Example 8: Unit-like structs without any fields
struct AlwaysEqual;
fn main3() {
    let subject = AlwaysEqual;
}

