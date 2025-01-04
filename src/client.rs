use std::{io::Write, net::UdpSocket};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    println!("Client started from {}", socket.local_addr()?);

    loop {
        println!("Enter message to send or 'quit' to exit: ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let message = input.trim();

        socket.send_to(message.as_bytes(), "127.0.0.1:34254")?;
        println!("Sent message: {}", message);

        let mut buf = [0u8; 1024];

        if message == "quit" {
            break;
        }

        let (amt, _) = socket.recv_from(&mut buf)?;
        println!(
            "Received response: {}",
            String::from_utf8_lossy(&buf[..amt])
        );
    }

    Ok(())
}
