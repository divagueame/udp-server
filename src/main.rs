use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    println!("init server at 2053");
    let socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 10];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!(" Size {:?},", size);
                println!("Source {:} ",  source);
                println!("buf {:?}",  buf);
                // let header = build_header(&[2; 10]);
                let message = DnsMessage::new();
                // let h = Header::new();

                println!("Message {:?}", message);
                println!("- - - - - - -  ");
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

#[derive(Debug)]
struct DnsMessage {
    header: Header
}
impl DnsMessage {
    fn new() -> Self {
        Self {
            header: Header::new()
        }
    }
}

#[derive(Debug)]
struct Header {
    id: u16,
    qr: u16,
    opcode: u16,
    aa: u16,
    tc: u16,
    rd: u16,
    ra: u16,
    z	: u16,
    rcode: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16
}

impl Header {
    fn new() -> Self {
        Self {
            id: 1234,
            qr: 1,
            opcode: 0,
            aa:0,
            tc:0,
            rd:0,
            ra:0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount:0,
            nscount:0,
            arcount:0 
        }
    }
}
