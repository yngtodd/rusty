fn main() {

}

struct Ipv4Addr {
    //
    addr: String
}

struct Ipv6Addr {
    //
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

enum message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32
}

// tuple structs
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);