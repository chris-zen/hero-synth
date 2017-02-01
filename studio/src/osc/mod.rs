use std::thread::{self, JoinHandle};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::Arc;
use std::str::FromStr;
use std::net::{UdpSocket, SocketAddrV4};

use rosc::{self, OscPacket};

pub struct Osc {
    address: SocketAddrV4,
    running: Arc<AtomicBool>,
    input_join_handler: Option<JoinHandle<()>>,
}

impl Osc {
    pub fn new(address: &str) -> Self {
        let addr = SocketAddrV4::from_str(address).unwrap();
        Osc {
            address: addr,
            running: Arc::new(AtomicBool::new(false)),
            input_join_handler: None,
        }
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn start(&mut self, sender: Sender<OscPacket>) {
        let running = self.running.swap(true, Ordering::Relaxed);
        if !running {
            let socket = UdpSocket::bind(self.address).unwrap();
            let running = self.running.clone();
            self.input_join_handler = Some(thread::spawn(move || {
                Self::input_loop(&running, &socket, sender)
            }));
        }
    }

    pub fn stop(&mut self) {
        let running = self.running.swap(false, Ordering::Relaxed);
        if running {
            self.input_join_handler.take().unwrap().join().ok();
        }
    }

    fn input_loop(
        running: &AtomicBool,
        socket: &UdpSocket,
        sender: Sender<OscPacket>) {

        let mut buf = [0u8; rosc::decoder::MTU];

        while running.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    // println!("Received packet with size {} from: {}", size, addr);
                    let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                    sender.send(packet).unwrap();
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                }
            }
        }
    }
}
