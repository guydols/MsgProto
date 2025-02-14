mod prelude;
mod error;
mod tcp;
mod message;
use std::{net::TcpStream, time::Duration};

use message::{HeartBeat, RootMsg};
use prelude::*;
use tcp::{receive_ftcp_stream, send_ftcp_stream};

fn main() {
    match client() {
        Ok(_) => todo!(),
        Err(e) => eprintln!("{}", e),
    };
}

fn client() -> Result<()>{
    let mut stream = match TcpStream::connect("127.0.0.1:32100") {
        Ok(stream) => stream,
        Err(_) => todo!(),
    };
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    stream.set_write_timeout(Some(Duration::from_secs(60)))?;
    stream.set_keepalive(Some(Duration::from_secs(60)))?;

    let mut index: u32 = 0;
    // Create and send message
    let msg = RootMsg::new(message::MessageType::HeartBeat(HeartBeat::new(index)));
    let data = bitcode::encode(&msg);
    send_ftcp_stream(&mut stream, &data)?;
    index = index.overflowing_add(1).0;

    loop {
        match receive_ftcp_stream(&mut stream) {
            Ok(received_data) => {
                let msg = bitcode::decode::<RootMsg>(&received_data)?;
                msg.handle()?;
            },
            Err(_) => continue,
        };

        let msg = RootMsg::new(message::MessageType::HeartBeat(HeartBeat::new(index)));
        let data = bitcode::encode(&msg);
        send_ftcp_stream(&mut stream, &data)?;
        index = index.overflowing_add(1).0;
    }
}
