use rustc_serialize::json;
use std::sync::mpsc::{Receiver, Sender};
use crate::player::{Player};

pub struct Client {
  pub server_address: String,
  pub server_port: String,
}

impl Client {
    fn new(&self, address: String, port: String) -> Self {
        return Self {
            server_address: address,
            server_port: port,
        };
    }

    fn get_conn_string(&self) -> String {
        return format!(
            "tcp://{address}:{port}",
            address = self.server_address,
            port = self.server_port
        );
    }

    pub fn send_to_server(&self, rx: Receiver<Player>) {
        // Receive from channel and send to server with zmq
        let player = rx.recv().unwrap();
        let encoded = json::encode(&player).unwrap();
        let mut msg = zmq::Message::new();
        println!("Connecting to Server...\n");

        let context = zmq::Context::new();
        let requester = context.socket(zmq::REQ).unwrap();

        assert!(requester.connect(self.get_conn_string().as_str()).is_ok());
        println!("## CLIENT ## Sending to server message = {}", encoded);
        // Send to zmq
        requester.send(encoded.as_str(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!(
            "## CLIENT ## Response from server = {}\n",
            msg.as_str().unwrap()
        );
    }
}
