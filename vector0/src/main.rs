#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let v = vec![
        IpAddr::V4("127.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];
    
    for ip in v {
        show_addr(ip)
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip)
}


