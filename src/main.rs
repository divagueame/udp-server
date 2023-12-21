use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    println!("init server at 34254");
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf = [0; 10];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Ok Size{:?}, Source {:}, buf {:?}", size, source, buf);
                let header = build_header(&[2; 10]);
                let res = [];
                socket
                    .send_to(&res, source)
                    .expect("Failed sending the message");
            },
            Err(e) => {
                println!("No!, {}", e);
                break Err(e);
            }
        }

    }
}


fn build_header(buff: &[u8; 10]) -> [u8; 10] {
    println!("HEADER: {:?}", *buff);
    *buff
}

struct Header {
    id: u16
    // QR: u16,
    // OPCODE: u16,
    // AA: u16,
    // TC: u16,
    // RD: u16,
    // RA: u16,
    // Z	Reserved: u16,
    // RCODE: u16,
    // QDCOUNT: u16,
    // ANCOUNT: u16,
    // NSCOUNT: u16,
//     ARCOUNT: u16
}
