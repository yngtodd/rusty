fn main() {

}

struct Ipv4Addr {
    //
    addr: String
}

struct Ipv6Addr {
    //
    addr: String
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

impl IpAddr {
    pub fn print(&self) {
        println!("")
    }
}