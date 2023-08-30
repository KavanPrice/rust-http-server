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
    }
}