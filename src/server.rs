use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("OK");
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
