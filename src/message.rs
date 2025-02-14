use bitcode::{Decode, Encode};

use crate::prelude::*;

// HeartBeat message
#[derive(Debug, Encode, Decode)]
pub struct HeartBeat {
    id: u32,
}

impl HeartBeat {
    pub fn new(id: u32) -> Self {
        HeartBeat { id }
    }
    
    pub fn handle(&self) -> Result<Option<MessageType>> {
        println!("Handling HeartBeat with id: {}", self.id);
        Ok(Some(MessageType::HeartBeat(HeartBeat::new(self.id + 1))))
    }
}

// Shutdown message
#[derive(Debug, Encode, Decode)]
pub struct Shutdown {
    reason: String,
}

impl Shutdown {
    pub fn new(reason: String) -> Self {
        Shutdown { reason }
    }
    
    pub fn handle(&self) -> Result<Option<MessageType>> {
        println!("Handling Shutdown: {}", self.reason);
        Ok(None)
    }
}

// Define message types as an enum that contains the actual messages
#[derive(Encode, Decode, Debug)]
pub enum MessageType {
    HeartBeat(HeartBeat),
    Shutdown(Shutdown),
}

impl MessageType {
    pub fn handle(&self) -> Result<Option<MessageType>> {
        match self {
            MessageType::HeartBeat(hb) => hb.handle(),
            MessageType::Shutdown(s) => s.handle(),
        }
    }
}

// Root message that contains the message type
#[derive(Encode, Decode)]
pub struct RootMsg {
    pub msg: MessageType,
}

impl RootMsg {
    pub fn new(msg: MessageType) -> Self {
        RootMsg { msg }
    }

    pub fn handle(&self) -> Result<Option<RootMsg>> {
        match self.msg.handle()? {
            Some(new_msg) => Ok(Some(RootMsg::new(new_msg))),
            None => Ok(None),
        }
    }
}
