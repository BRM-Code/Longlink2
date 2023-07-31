use std::net::{SocketAddr, UdpSocket};

const IP: &str = "127.0.0.1";
const PORT_UP: u16 = 1730;
// const PORT_DOWN: u16 = 1735;
const BUFFER_SIZE: usize = 1024;

pub fn listen() { // TODO: Does this function block the port? is that a problem?
    println!("Listening for Packets on {}:{}", IP, PORT_UP);
    // Setup socket and buffer
    let socket = UdpSocket::bind(format!("{}:{}", IP, PORT_UP)).expect("Failed to bind UDP socket");
    let mut buffer = [0u8; BUFFER_SIZE];

    // Collect data from the socket and store the size
    let (packet_size, src_addr) = socket.recv_from(&mut buffer).expect("Failed to receive UDP packet");

    // Collect the type of data from the packet for processing
    let packet_id = &buffer[..4];

    // Organise the packet to process the data
    sort_packet(packet_id, src_addr, socket);

    // Collect the message from the packet and convert it to a string
    let message = String::from_utf8_lossy(&buffer[12..packet_size]).to_string();
    println!("Received message: {}", message);
}

fn sort_packet(packet_id: &[u8], src_addr: SocketAddr, socket: UdpSocket) {
    let packet_type = packet_id[3];
    let token = &packet_id[1..3];

    println!("->[{:?}] Packet Signature", &packet_id[..4]);

    match packet_type {
        0x00 => { // PUSH_DATA Packet
            ack_pktfwd(token, 1, src_addr, socket);
        }
        0x02 => { // PULL_DATA Packet
            ack_pktfwd(token, 4, src_addr, socket);
        }
        0x05 => { // TX_ACK Packet
            ack_pktfwd(token, 5, src_addr, socket);
        }
        _ => { // UNKNOWN Packet
            println!("Unknown Packet! Packet ID: {:?}", packet_id);
        }
    }
}

fn ack_pktfwd(token: &[u8], response_type: u8, src_addr: SocketAddr, socket: UdpSocket) {
    let message: &[u8; 4] = &[2, token[0], token[1], response_type];
    socket.send_to(message, src_addr).expect("[ERROR] couldn't acknowledge the pk-fr");

    // Print the message that is being sent
    println!("<-[{:?}] Sending ACK packet to {}", message, src_addr);
}
