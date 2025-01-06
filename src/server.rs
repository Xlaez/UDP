use serde::Serialize;
use std::{
    net::UdpSocket,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    match connect() {
        Ok(_) => println!("Server shutdown gracefully"),
        Err(e) => println!("error: {:?}", e),
    }
}

#[derive(Serialize, Clone)]
struct Msg {
    pub text: String,
    pub timestamp: u64,
}

impl Msg {
    fn new(text: &str, timestamp: u64) -> Msg {
        Msg {
            text: text.to_string(),
            timestamp,
        }
    }
}

fn connect() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        println!("Server listening on 127.0.0.1:34254");

        let mut msgs: Vec<Msg> = Vec::with_capacity(50);

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold the message, it will be cut off.
            let mut buf = [0u8; 1024];

            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    println!("Received {} bytes from {} ", amt, src);

                    let string_data = String::from_utf8_lossy(&buf[..amt]);

                    println!("Data: {:?}", string_data.clone());

                    let data = Msg::new(
                        &string_data,
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    );

                    msgs.push(data.clone());

                    if string_data == "quit" {
                        break;
                    }

                    if string_data == "my_data" {
                        let serialized_data =
                            serde_json::to_vec(&msgs).expect("Could not serialize response data");
                        socket.send_to(&serialized_data, src)?;
                    } else {
                        let serialized_data = serde_json::to_string(&data)
                            .expect("Could not serialize response data");
                        socket.send_to(&serialized_data.as_bytes(), &src)?;
                    }

                    println!("Sent response back to {}", src);
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    break;
                }
            }
        }
    }
    Ok(())
}
