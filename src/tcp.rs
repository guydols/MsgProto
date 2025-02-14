use crate::prelude::*;
use std::{io::{Read, Write}, net::TcpStream};

pub fn send_ftcp_stream(stream: &mut TcpStream, data: &[u8]) -> Result<()> {
    let len = data.len();
    stream.write_all(&len.to_be_bytes())?;
    Ok(stream.write_all(data)?)
}

pub fn receive_ftcp_stream(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut len_bytes = [0; 8];
    stream.read_exact(&mut len_bytes)?;
    let len = usize::from_be_bytes(len_bytes);
    let mut data = vec![0; len];
    stream.read_exact(&mut data)?;
    Ok(data)
}