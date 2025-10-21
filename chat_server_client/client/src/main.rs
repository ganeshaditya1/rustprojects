use sprintf::sprintf;
use std::env;
use std::io::{self, Read};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedReadHalf;

fn read_cli_args() -> (String, u16) {
    // args: program name, address, port
    let mut args = env::args().skip(1); // skip program name

    let address = args.next().expect("Usage: prog <address> <port>");
    let port = args.next().expect("Usage: prog <address> <port>");

    // optional: validate port as number
    let port: u16 = port.parse().expect("port must be a number 0..65535");
    (address, port)
}

async fn client_reader(mut read_stream: OwnedReadHalf) {
    let mut buf = vec![0u8; 1024];
    loop {
        let n = read_stream.read(&mut buf).await.unwrap();
        println!("Received msg: {}", n);
        if n == 0 { break }
        println!("{}", String::from_utf8_lossy(&buf[..n]));
    }
}


async fn start_client(addr: String, portno: u16) {
    let connection_str = sprintf!("%s:%d", addr, portno).unwrap();
    let stream = TcpStream::connect(connection_str).await.unwrap();
    let (reader, mut writer) = stream.into_split();
    tokio::spawn(client_reader(reader));
    println!("Started client listener.");

    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_to_string(&mut name).unwrap();
    writer.write_all(&name.as_bytes()).await.unwrap();

    loop {
        let mut message = String::new();
        io::stdin().read_to_string(&mut message).unwrap();
        message = sprintf!("[%s]: %s", name.trim(), message.trim()).unwrap();
        writer.write_all(&message.as_bytes()).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (server_addr, portno) = read_cli_args();

    start_client(server_addr, portno).await;
} 