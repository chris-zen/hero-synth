use std::thread::{self, JoinHandle};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use std::net::SocketAddr;
use std::io;

use futures::{self, Future, Stream, Sink};
use tokio_core::net::{UdpSocket, UdpCodec};
use tokio_core::reactor::Core;

use rosc::{self, OscError, OscPacket};

pub struct OscCodec;

impl UdpCodec for OscCodec {
    type In = (SocketAddr, OscPacket);
    type Out = (SocketAddr, OscPacket);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        rosc::decoder::decode(buf)
            .map(|packet| (*addr, packet))
            .or_else(|err| match err {
                OscError::ReadError(io_err) => Err(io_err),
                osc_err => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{:?}", osc_err)))
            })
    }

    fn encode(&mut self, (addr, packet): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        rosc::encoder::encode(&packet)
            .map(|data| into.extend(data)).ok();
        addr
    }
}

pub struct Osc {
    address: SocketAddr,
    running: Arc<AtomicBool>,
    input_join_handler: Option<JoinHandle<()>>,
    output_join_handler: Option<JoinHandle<()>>,
}

impl Osc {
    pub fn new(address: &str) -> Self {
        let addr = SocketAddr::from_str(address).unwrap();
        Osc {
            address: addr,
            running: Arc::new(AtomicBool::new(false)),
            input_join_handler: None,
            output_join_handler: None,
        }
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn start(&mut self, sender: Sender<OscPacket>, receiver: Receiver<OscPacket>) {
        let running = self.running.swap(true, Ordering::Relaxed);
        if !running {
            // let socket = Arc::new(Mutex::new(UdpSocket::bind(self.address).unwrap()));
            // let socket = UdpSocket::bind(self.address).unwrap();
            // let running = self.running.clone();
            // self.input_join_handler = Some(thread::spawn(move || {
            //     Self::input_loop(&running, &socket, sender)
            // }));
            // let running = self.running.clone();
            // // let address = self.address;
            // self.output_join_handler = Some(thread::spawn(move || {
            //     Self::output_loop(&running, &socket, receiver)
            // }));

            let address = self.address.clone();
            thread::spawn(move || {
                let mut core = Core::new().unwrap();
                let handle = core.handle();

                let socket = UdpSocket::bind(&address, &handle).unwrap();
                let local_addr = socket.local_addr()
                    .map(|addr| format!("{}", addr))
                    .unwrap_or("unknown".to_string());
                println!("Listening for OSC at {} ...", local_addr);

                let (sink, stream) = socket.framed(OscCodec).split();

                let osc_input = stream.and_then(|(_addr, packet)| {
                    // println!("{:?} -> {:?}", _addr, packet);
                    sender.send(packet).or_else(|_| {
                        Err(io::Error::new(io::ErrorKind::Other, "OSC sender channel disconnected"))
                    })
                });

                // fn pk(p: OscPacket) -> Result<(SocketAddr, OscPacket), io::Error> {
                //     let addr: SocketAddr = SocketAddr::from_str("255.255.255.255:8080").unwrap();
                //     Ok((addr.clone(), p))
                // }
                //
                // let packets = receiver.into_iter().map(|p: OscPacket| pk(p));
                // let packets_stream = futures::stream::iter(packets).or_else(|_| {
                //     Err(io::Error::new(io::ErrorKind::Other, "???"))
                // });
                // let osc_output = sink.send_all(packets_stream);
                // handle.spawn(osc_output.then(|_| Ok(())));

                // fn process(v: usize) -> Result<(), ()> {
                //     println!("Computing {}", v);
                //     use std::thread;
                //     use std::time::Duration;
                //     thread::sleep(Duration::from_millis(1000));
                //     Ok(())
                // }
                // use futures::stream;
                // let s2 = stream::repeat(2)
                //             .map(process)
                //             // .take(20)
                //             .for_each(|_| Ok(()));
                // handle.spawn(s2);
                drop(core.run(osc_input.for_each(|_| Ok(()))));
            });
        }
    }

    pub fn stop(&mut self) {
        let running = self.running.swap(false, Ordering::Relaxed);
        if running {
            self.input_join_handler.take().unwrap().join().ok();
        }
    }

    // fn input_loop(
    //     running: &AtomicBool,
    //     socket: &UdpSocket,
    //     sender: Sender<OscPacket>) {
    //
    //     let mut buf = [0u8; rosc::decoder::MTU];
    //
    //     while running.load(Ordering::Relaxed) {
    //         match socket.recv_from(&mut buf) {
    //             Ok((size, addr)) => {
    //                 // println!("Received packet with size {} from: {}", size, addr);
    //                 let packet = rosc::decoder::decode(&buf[..size]).unwrap();
    //                 sender.send(packet).unwrap();
    //             }
    //             Err(e) => {
    //                 println!("Error receiving from socket: {}", e);
    //             }
    //         }
    //     }
    // }
    //
    // fn output_loop(
    //     running: &AtomicBool,
    //     socket: &UdpSocket,
    //     // address: &str,
    //     receiver: Receiver<OscPacket>) {
    //
    //     while running.load(Ordering::Relaxed) {
    //         let packet = receiver.recv().unwrap();
    //         let buf = rosc::encoder::encode(&packet).unwrap();
    //         socket.send(&buf).unwrap();
    //     }
    // }

    // fn input_loop(
    //     running: &AtomicBool,
    //     socket: &UdpSocket,
    //     sender: Sender<OscPacket>) {
    //
    //     let mut buf = [0u8; rosc::decoder::MTU];
    //
    //     while running.load(Ordering::Relaxed) {
    //         match socket.recv_from(&mut buf) {
    //             Ok((size, addr)) => {
    //                 // println!("Received packet with size {} from: {}", size, addr);
    //                 let packet = rosc::decoder::decode(&buf[..size]).unwrap();
    //                 sender.send(packet).unwrap();
    //             }
    //             Err(e) => {
    //                 println!("Error receiving from socket: {}", e);
    //             }
    //         }
    //     }
    // }
    //
    // fn output_loop(
    //     running: &AtomicBool,
    //     socket: &UdpSocket,
    //     // address: &str,
    //     receiver: Receiver<OscPacket>) {
    //
    //     while running.load(Ordering::Relaxed) {
    //         let packet = receiver.recv().unwrap();
    //         let buf = rosc::encoder::encode(&packet).unwrap();
    //         socket.send(&buf).unwrap();
    //     }
    // }
}
