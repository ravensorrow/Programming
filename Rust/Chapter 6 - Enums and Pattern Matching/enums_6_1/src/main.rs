#![allow(unused)]
fn main() {
    struct Ipv4Addr {
        // -- snip --
    }

    struct Ipv6Addr {
        // -- snip --
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    
    let loopback = IpAddr::V6(String::from("::1"));

}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage { 
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct CHangeColorMessage(i32, i32, i32); // tuple struct

fn main1() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // Method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
