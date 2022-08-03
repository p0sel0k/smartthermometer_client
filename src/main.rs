use std::{io, sync::Mutex, thread, time::Duration};

use anyhow::{Context, Result};
use smartthermometer_client::*;

fn main() -> Result<()> {
    // let thermo_ref = thermo.clone();
    loop {
        let mut command = String::new();
        if let Err(e) = io::stdin().read_line(&mut command) {
            return Err(ThermometerClientError::Io(e)).context("can't read command");
        }
        match command.as_str().trim() {
            "temperature" => listen_udp(),
            _ => return Ok(()),
        }
    }
}

fn listen_udp() {
    thread::spawn(move || {
        let thermo = Mutex::new(ThermometerClient::connect_to_socket("127.0.0.1:8095").unwrap());
        println!("thread has been started");
        loop {
            // let thermo_locked = thermo_ref.lock().unwrap();
            let thermo_locked = thermo.lock().unwrap();
            let temp = thermo_locked.recieve_temperature().unwrap();
            println!("recieved data: '{}'", temp);
            thread::sleep(Duration::from_secs(1));
        }
    });
}
