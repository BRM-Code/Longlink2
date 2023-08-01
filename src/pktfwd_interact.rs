use std::net::{SocketAddr, UdpSocket};
use std::ptr::null;
use json::parse;
use openssl::symm::{decrypt, Cipher};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};


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

            if parsed.has_key("rxpk"){
                let data = parsed["rxpk"][0]["data"].to_string();
                let mut data_decoded = general_purpose::STANDARD.decode(data).unwrap();

                //println!("data decoded: {:?}", std::str::from_utf8(&data_decoded).unwrap());

                let uav_id = &data_decoded[1..3];
                println!("UAV ID: {:?}", std::str::from_utf8(uav_id).unwrap());

                let key = [0x2B, 0x7E, 0x15, 0x16, 0x28, 0xAE, 0xD2, 0xA6, 0xAB, 0xF7, 0x15, 0x88, 0x09, 0xCF, 0x4F, 0x3C];
                let iv:[u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

                let extra = data_decoded.drain(4..30);

                println!("Decrypting {:?}", &extra);

                let decrypted = decrypt(
                    Cipher::aes_128_cbc(),
                    &key,
                    Some(&iv),
                    (&extra).as_ref()
                );
                println!("Decrypted: {:?}", decrypted);
            }



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
