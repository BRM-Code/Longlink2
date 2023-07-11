use std::net::UdpSocket;

const IP: &str = "127.0.0.1";
const PORT: u16 = 1730;
const BUFFER_SIZE: usize = 1024;

pub fn listen() { // TODO: Does this function block the port? is that a problem?
    println!("Listening for Packets on {}:{}", IP, PORT);
    // Setup socket and buffer
    let socket = UdpSocket::bind(format!("{}:{}", IP, PORT)).expect("Failed to bind UDP socket");
    let mut buffer = [0u8; BUFFER_SIZE];

    // Collect data from the socket and store the size
    let (packet_size, _) = socket.recv_from(&mut buffer).expect("Failed to receive UDP packet");

    // Collect the type of data from the packet for processing
    let packet_id = buffer[3];

    sort_packet(packet_id);

    // Collect the message from the packet and convert it to a string
    let message = String::from_utf8_lossy(&buffer[3..packet_size]).to_string();
    println!("Received message: {}", message);
}

fn sort_packet(packet_id : u8) {
    match packet_id {
        0x00 => { // PUSH_DATA Packet

        }
        0x02 => { // PULL_DATA Packet

        }
        0x05 => { // TX_ACK Packet

        }

        _ => println!("Unknown Packet!")
    }
}
