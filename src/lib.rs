use anyhow::Result;
use async_io::udp::{recv_async, send_async};
use std::io::{self};
use thiserror::Error;
use tokio::net::{ToSocketAddrs, UdpSocket};

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
    pub async fn connect_to_socket<Addr>(addr: Addr) -> ThermometerClientResult<Self>
    where
        Addr: ToSocketAddrs,
    {
        let udp = UdpSocket::bind("127.0.0.1:8096").await?;
        udp.connect(addr).await?;
        Self::try_handshake(udp).await
    }

    async fn try_handshake(udp: UdpSocket) -> ThermometerClientResult<Self> {
        send_async(&udp, b"smart").await?;
        let mut buf = [0; 4];
        recv_async(&udp, &mut buf).await?;
        if &buf != b"home" {
            let msg = format!("recieved string is: {:?}", buf);
            return Err(ThermometerClientError::BadHandshake(msg));
        }
        println!("Succesfully connected!");
        Ok(Self { udp })
    }

    pub async fn recieve_temperature(&self) -> ThermometerClientResult<String> {
        let mut buf = [0; 4];
        recv_async(&self.udp, &mut buf).await?;
        let len = u32::from_be_bytes(buf);
        let mut temp = vec![0; len as _];
        recv_async(&self.udp, &mut temp).await?;
        String::from_utf8(temp).map_err(|_| ThermometerClientError::BadEncoding)
    }
}
