#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    route(IpAddrKind::V4);
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}
