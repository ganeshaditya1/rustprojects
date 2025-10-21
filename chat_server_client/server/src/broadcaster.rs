use tokio::net::tcp::OwnedWriteHalf;
use tokio::io::{AsyncWriteExt};
use tokio::sync::mpsc;

// The broadcaster can receive two kinds of commands.
// 1. A message to broadcast to all clients.
// 2. A socket which denotes a new client that has joined the group.
// This new socket needs to be added to the list of sockets to which we need to 
// Broad cast message to.

#[derive(Debug)]
pub enum BroadcasterCmd {
    Msg([u8; 1024]),
    Socket(OwnedWriteHalf),
}
    
pub struct Broadcaster {
    mpsc_channel: mpsc::Receiver<BroadcasterCmd>,
    outgoing_sockets: Vec<OwnedWriteHalf>,
}

impl Broadcaster {
    pub fn new(mpsc_channel: mpsc::Receiver<BroadcasterCmd>) -> Broadcaster {
        return Broadcaster {
            mpsc_channel: mpsc_channel,
            outgoing_sockets: Vec::new(),
        }
    }

    pub async fn start_listener(mut self) {
        loop {
            let broadcast_cmd = self.mpsc_channel.recv().await.unwrap();
            match broadcast_cmd {
                BroadcasterCmd::Socket(socket_stream) => self.outgoing_sockets.push(socket_stream),
                BroadcasterCmd::Msg(data) => {
                    for socket in &mut self.outgoing_sockets {
                        let _ = socket.write(&data).await;
                    }
                }
            }            
        }
    }
}