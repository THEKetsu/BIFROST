use std::net::Ipv4Addr;
use ipnetwork::{IpNetwork, Ipv4Network};
use local_ip_address::local_ip;

fn show_ip()->std::net::IpAddr{
    let my_local_ip = local_ip().unwrap();
    println!("This is my local IP address: {:?}", my_local_ip);
    my_local_ip
}


pub async  fn other() {
    let my_local_ip=show_ip();
    let net: Ipv4Network = "127.0.0.1/16".parse().unwrap();
    assert_eq!(net.broadcast(), Ipv4Addr::new(127, 0, 255, 255));
}