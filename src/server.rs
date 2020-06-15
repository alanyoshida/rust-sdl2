use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use rustc_serialize::json;
use std::time::Duration;
use crate::player::{Player};

//*:5555
pub struct Server {
  pub address: String,
  pub port: String,
}

impl Server {
    fn get_conn_string(&self) -> String {
        return format!(
            "tcp://{address}:{port}",
            address = self.address,
            port = self.port
        );
    }

    pub fn new() -> Self {
        return Self {
            address: String::from("*"),
            port: String::from("5555"),
        };
    }

    pub fn start(&self, tx: mpsc::Sender<Player>) {
        // Receive from zmq and send to channel
        println!("Initialing ZeroMQ server ...");
        let context = zmq::Context::new();
        let responder = context.socket(zmq::REP).unwrap();

        assert!(responder.bind(self.get_conn_string().as_str()).is_ok());

        let mut msg = zmq::Message::new();
        loop {
            responder.recv(&mut msg, 0).unwrap();

            let decoded: Player = json::decode(msg.as_str().unwrap()).unwrap();
            println!("## SERVER ## Message received from client = {:?}", decoded);

            // Send to channel
            tx.send(decoded).unwrap();
            thread::sleep(Duration::from_millis(1000));
            responder.send("OK", 0).unwrap();
        }
    }
}
