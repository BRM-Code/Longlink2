mod blockchain;
mod pktfwd_interact;

fn main() {
    println!("LongLink Start");
    loop {
        pktfwd_interact::listen();
    }
}
