use libp2p::identify::Identify;
use libp2p::NetworkBehaviour;
use libp2p::ping::Ping;

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    identify: Identify,
    ping: Ping,
}

fn main() {
    println!("Hello, world!");
}
