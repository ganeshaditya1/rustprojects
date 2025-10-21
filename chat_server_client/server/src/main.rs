use std::env;

mod broadcaster;
mod listener;

fn read_cli_args() -> (String, u16) {
    // args: program name, address, port
    let mut args = env::args().skip(1); // skip program name

    let address = args.next().expect("Usage: prog <address> <port>");
    let port = args.next().expect("Usage: prog <address> <port>");

    // optional: validate port as number
    let port: u16 = port.parse().expect("port must be a number 0..65535");
    (address, port)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let (address, portno) = read_cli_args();
    listener::start_server(address, portno).await;
}
