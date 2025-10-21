use tokio::net::TcpListener;
use tokio::net::tcp::OwnedReadHalf;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use sprintf::sprintf;

use crate::broadcaster::Broadcaster;
use crate::broadcaster::BroadcasterCmd;


pub async fn socket_reader(mut socket: OwnedReadHalf, 
                       mpsc_channel: mpsc::Sender<BroadcasterCmd>) {
    let mut buf = [0; 1024];
    loop {
        let n = socket.read(&mut buf).await.unwrap();
        if n == 0 {continue;}
        let payload = BroadcasterCmd::Msg(buf);
        mpsc_channel.send(payload).await.unwrap();
    }
}

pub async fn start_server(srv_adr: String, portno: u16) {
    let address = sprintf!("%s:%d", srv_adr, portno).unwrap();

    // Communication channel for all the async tasks.
    let (tx, rx) = mpsc::channel::<BroadcasterCmd>(8);

    let listener = TcpListener::bind(address).await.unwrap();
    let broadcaster = Broadcaster::new(rx);
    tokio::spawn(broadcaster.start_listener());
    println!("Server started.");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let (read_stream, write_stream) = socket.into_split();

        // Let the broadcaster know that a new client has joined the group.
        tx.send(BroadcasterCmd::Socket(write_stream)).await.unwrap();

        tokio::spawn(socket_reader(read_stream, tx.clone()));
    }
}
