use std::env;
use std::net::{IpAddr, ToSocketAddrs};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <addr>", args[0]);
        process::exit(1);
    }

    let host = &args[1];
    match get_host_info(host) {
        Ok(info) => print_host_info(&info),
        Err(e) => error_handling(&e.to_string()),
    }
}

fn get_host_info(host: &str) -> Result<IpAddr, Box<dyn std::error::Error>> {
    let addr = format!("{}:80", host)
        .to_socket_addrs()?
        .next()
        .ok_or("Failed to resolve address")?;
    Ok(addr.ip())
}

fn print_host_info(ip: &IpAddr) {
    println!("Official name: {}", ip);
    println!("Aliases: None (not suported in this unimplemention)");
    println!(
        "Address type: {}",
        match ip {
            IpAddr::V4(_) => "AF_INET",
            IpAddr::V6(_) => "AF_INET6",
        }
    );
    println!("IP addr: {}", ip);
}

fn error_handling(message: &str) {
    eprintln!("{}", message);
    process::exit(1);
}
