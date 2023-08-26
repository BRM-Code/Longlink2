use std::net::{SocketAddr, UdpSocket};
use json::parse;
use openssl::symm::{Cipher, Crypter, Mode};
use base64::{Engine as _, engine::general_purpose};
use crate::packet;


const IP: &str = "127.0.0.1";
const PORT_UP: u16 = 1730;
// const PORT_DOWN: u16 = 1735;
const BUFFER_SIZE: usize = 1024;

//Received message: {"rxpk":[
// {"jver":1,"tmst":3829580819,"chan":5,"rfch":0,
// "freq":867.500000,"mid": 8,"stat":1,"modu":"LORA","datr":"SF7BW125",
// "codr":"4/5","rssis":-43,"lsnr":9.5,"foff":4371,"rssi":-42,"size":59,
// "data":"I3UxwTmJrbajOHOdjW4V/en9cC18us1ErRcuYAYT6em6v20AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="}]}

pub fn listen() { // TODO: Does this function block the port? is that a problem?
    println!("Listening for Packets on {}:{}", IP, PORT_UP);

    // Setup socket and buffer
    let socket = UdpSocket::bind(format!("{}:{}", IP, PORT_UP)).expect("Failed to bind UDP socket");
    let mut buffer = [0u8; BUFFER_SIZE];

    // Collect data from the socket and store the size
    let (packet_size, src_addr) = socket.recv_from(&mut buffer).expect("Failed to receive UDP packet");

    // Collect the type of data from the packet for processing
    let packet_id = &buffer[..4];

    // Collect the message from the packet and convert it to a string
    let message = String::from_utf8_lossy(&buffer[12..packet_size]).to_string();
    println!("Received message: {}", &message);

    // Organise the packet to process the data
    sort_packet(packet_id, src_addr, socket, message);
}

fn sort_packet(packet_id: &[u8], src_addr: SocketAddr, socket: UdpSocket, message : String) {
    let packet_type = packet_id[3];
    let token = &packet_id[1..3];

    println!("->[{:?}] Packet Signature", &packet_id[..4]);

    match packet_type {
        0x00 => { // PUSH_DATA Packet
            ack_pktfwd(token, 1, src_addr, socket);

            let parsed = parse(&message).unwrap();
            let contains_data = parsed.has_key("rxpk");
            let contains_stats = parsed.has_key("stat");
            if contains_data {
                //packet::process_data_packet(parsed)
            }
            if contains_stats {
                println!("[PKFWD] Received stat packet");
                // TODO: maybe check values received are healthy, temp etc
            }
            if !contains_data && !contains_stats {
                println!("[PKFWD] Received PUSH_DATA packet -> JSON incorrect");
            }
        }
        0x02 => { // PULL_DATA Packet
            ack_pktfwd(token, 4, src_addr, socket);
            println!("[PKFWD] Received PULL_DATA packet -> Network route open");
            /* TODO: this message can get annoying, maybe only message
                when one of these packets haven't been received in a while */
        }
        0x05 => { // TX_ACK Packet
            ack_pktfwd(token, 5, src_addr, socket);
            println!("[PKFWD] Received TX_ACK packet -> Packet sent successfully!");
        }
        _ => { // UNKNOWN Packet
            println!("[PKFWD] Received Unknown packet Packet ID: {:?}", packet_id);
            // TODO: maybe log these packets for analysis
        }
    }
}

fn ack_pktfwd(token: &[u8], response_type: u8, src_addr: SocketAddr, socket: UdpSocket) {
    let message: &[u8; 4] = &[2, token[0], token[1], response_type];
    socket.send_to(message, src_addr).expect("[ERROR] couldn't acknowledge the pk-fr");

    // Print the message that is being sent
    println!("<-[{:?}] Sending ACK packet to {}", message, src_addr);
}

struct TelemetryData {
    latitude: f32,
    longitude: f32,
    vbatt: f32,
    altitude: u8,
    ground_speed: u8,
    satellites: u8,
    consumption: u8,
    rssi: u8,
    pitch: i16,
    roll: i16,
    heading: i16,
    arm: bool,
    sat_fix: bool,
}


#[cfg(test)]
mod tests {
    #[test]
    fn ack_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn ack_test2() {
        let result = 2 + 2;
        assert_eq!(result, 3);
    }
}