use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    println!("init server at 2053");
    let socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let message = DnsMessage::new();

                println!("Message {:?}", &message.encode());
                socket
                    .send_to(&message.encode(), source)
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
    fn encode(&self) -> [u8;12] {
        let id = &self.header.id.to_be_bytes();
        let qdcount = &self.header.qdcount.to_be_bytes();
        let ancount = &self.header.ancount.to_be_bytes();
        let nscount = &self.header.nscount.to_be_bytes();
        let arcount = &self.header.arcount.to_be_bytes();
        let mut bytes: [u8;12] = [0b0000_000; 12];

        bytes[0] = id[0]; 
        bytes[1] = id[1];

        let mut byte2: u8 = 0b0000_0000;
        byte2 |= (self.header.qr as u8) << 7;
        byte2 |= (self.header.opcode as u8) << 3;
        byte2 |= (self.header.aa as u8) << 2;
        byte2 |= (self.header.tc as u8) << 1;
        byte2 |= (self.header.rd as u8) << 0;
        bytes[2] = byte2;

        let mut byte3: u8 = 0b0000_0000;
        byte3 |= (self.header.ra as u8) << 7;
        byte3 |= (self.header.z) << 4;
        byte3 |= (self.header.rcode) << 0;
        bytes[3] = byte3;

        bytes[4] = qdcount[0];
        bytes[5] =  qdcount[1];
        bytes[6] =  ancount[0];
        bytes[7] =  ancount[1];
        bytes[8] =  nscount[0];
        bytes[9] = nscount[1];
        bytes[10] = arcount[0];
        bytes[11] = arcount[1];
        bytes
    }
}

#[derive(Debug)]
struct Header {
    id: u16,
    qr: bool,
    opcode: u8,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16
}

impl Header {
    fn new() -> Self {
        Self {
            id: 1234,
            qr: true,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount:0,
            nscount:0,
            arcount:0 
        }
    }
}
