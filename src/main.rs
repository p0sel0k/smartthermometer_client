use anyhow::{Context, Result};
use smartthermometer_client::*;
use std::io::Write;
use tokio::{
    self,
    io::{self, AsyncBufReadExt},
    join,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "input command \n1) 'temperature' to get temperature \n2) anything else to close program):"
    );
    std::io::stdout().flush()?;
    loop {
        let mut command = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut command) {
            return Err(ThermometerClientError::Io(e)).context("can't read command");
        }
        match command.as_str().trim() {
            "temperature" => {
                let (_, _) = join!(listen_udp(), read_input());
            }
            _ => return Ok(()),
        }
    }
}

async fn listen_udp() -> Result<()> {
    let thermo = ThermometerClient::connect_to_socket("127.0.0.1:8095").await?;
    println!("connected to server");
    loop {
        let temp = thermo.recieve_temperature().await?;
        println!("recieved data: '{}'", temp);
    }
}

async fn read_input() -> Result<()> {
    let mut command = Vec::new();
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    loop {
        reader.read_until(b'\n', &mut command).await?;
        let command = std::str::from_utf8(command.as_slice())?;
        match command.trim() {
            "exit" => panic!("client died because of exit"),
            _ => panic!("client died"),
        }
    }
}
