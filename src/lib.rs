use anyhow::Result;
use std::{
    io::{self},
    net::{ToSocketAddrs, UdpSocket},
};
use thiserror::Error;

pub type ThermometerClientResult<T> = Result<T, ThermometerClientError>;

#[derive(Debug, Error)]
pub enum ThermometerClientError {
    #[error("Unexpected handshake: {0}")]
    BadHandshake(String),

    #[error("Io error")]
    Io(#[from] io::Error),

    #[error("BadEncoding")]
    BadEncoding,
}

pub struct ThermometerClient {
    udp: UdpSocket,
}

impl ThermometerClient {
    pub fn connect_to_socket<Addr>(addr: Addr) -> ThermometerClientResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let udp = UdpSocket::bind("127.0.0.1:8096")?;
        udp.connect(addr)?;
        Self::try_handshake(udp)
    }

    fn try_handshake(udp: UdpSocket) -> ThermometerClientResult<Self> {
        udp.send(b"smart")?;
        let mut buf = [0; 4];
        udp.recv(&mut buf)?;
        if &buf != b"home" {
            let msg = format!("recieved string is: {:?}", buf);
            return Err(ThermometerClientError::BadHandshake(msg));
        }
        println!("Succesfully connected!");
        Ok(Self { udp })
    }

    pub fn recieve_temperature(&self) -> ThermometerClientResult<String> {
        let mut buf = [0; 4];
        self.udp.recv(&mut buf)?;
        let len = u32::from_be_bytes(buf);
        let mut temp = vec![0; len as _];
        self.udp.recv(&mut temp)?;
        String::from_utf8(temp).map_err(|_| ThermometerClientError::BadEncoding)
    }
}
