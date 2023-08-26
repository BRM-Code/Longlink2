mod blockchain;
mod pktfwd_interact;
mod packet;

fn main() {
    println!("LongLink Start");
    loop {
        pktfwd_interact::listen();
    }
}
