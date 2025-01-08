use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Bytes of packet as 2-character hex values.
    hex: String,
}

fn main() {
    match hex::decode(Args::parse().hex) {
        Ok(bytes) => match simple_dns::Packet::parse(&bytes) {
            Ok(packet) => {
                dbg!(packet);
            }
            Err(err) => {
                println!("Error: failed to parse packet: {err}");
            }
        },
        Err(err) => {
            println!("Error: failed to parse hex bytes from input string: {err}")
        }
    };
}
