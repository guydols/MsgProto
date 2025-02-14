mod prelude;
mod error;
mod tcp;
mod message;
use std::{net::{SocketAddr, TcpListener, TcpStream}, time::Duration};

use message::RootMsg;
use prelude::*;
use tcp::{receive_ftcp_stream, send_ftcp_stream};

fn main() {
    let bind_to = f!("{}:{}", "127.0.0.1", "32100");
    let listener = match TcpListener::bind(bind_to) {
        Ok(v) => v,
        Err(_) => todo!(),
    };
    loop {
        match listen(&listener) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }
}

fn listen(listener: &TcpListener) -> Result<()> {
    let (stream, socket) = listener.accept()?;
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    stream.set_write_timeout(Some(Duration::from_secs(60)))?;
    match new_connection(stream, socket) {
        Ok(_) => (),
        Err(_) => {
            // Logging of failed client setup
            // let _ = stream.shutdown(Shutdown::Both);
            "Todo error".to_string();
        }
    }
    // Todo implement gracefull shutdown of listener
    Ok(())
}

fn new_connection( mut stream: TcpStream, socket: SocketAddr) -> Result<()> {
    let received_data = receive_ftcp_stream(&mut stream)?;
    let msg = bitcode::decode::<RootMsg>(&received_data)?;
    let response = msg.handle()?;


    // Check if there is an answer, encode it and send it
    match response {
        Some(data) => {
            let output = bitcode::encode(&data);
            send_ftcp_stream(&mut stream, &output)?;
        },
        None => todo!(),
    };
    Ok(())
}