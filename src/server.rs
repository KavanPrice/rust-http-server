use std::io::prelude::*;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // allow passing of string or string slice
    pub fn new(addr: impl Into<String>) -> Self {
        Self {addr: String::from(addr.into())}
    }

    pub fn run(self) {
        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        // accept incoming http connections
        loop {
            match listener.accept() {
                Ok((mut stream,addr)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => println!("Recieved on address {}: {}", addr, String::from_utf8_lossy(&buffer)),
                        Err(e) => println!("Error on reading stream from address {}: {}", addr, e)
                    }
                },
                Err(e) => println!("Error on trying to connect: {}", e)
            }
        }
    }
}