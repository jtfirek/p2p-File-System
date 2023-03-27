use std::{collections::HashSet};
use libp2p::{identity::Keypair, Multiaddr, PeerId, kad::{Kademlia, store::MemoryStore}};

// The Network struct represents the entire p2p network.
pub struct Network {
    // Local node's keypair
    keypair: Keypair,

    //local peer struct
    local_peer: Peer,

    // Hashset to store all the connected peers
    connected_peers: HashSet<Peer>,

    // DHT instance
    dht: Kademlia<MemoryStore>,

}

impl Network {
    pub fn new(keypair: Keypair, listen_address: Multiaddr) -> Self {
        // create a Peer instance from the public key and listen address
        let local_peer = Peer::new(PeerId::from(keypair.public()), listen_address.clone());
        // create a hashset to store the connected peers
        let connected_peers = HashSet::new();
        // create a DHT instance and ping the network
        let dht = Kademlia::new(local_peer.id.clone(), MemoryStore::new(local_peer.id.clone()));
        Self {
            keypair,
            local_peer,
            connected_peers,
            dht,
        }
    }
}


pub struct Peer {
    // unique identifier for a peer in the network
    pub id: PeerId,
    // IP address and port of the peer
    pub address: Multiaddr,
}

impl Peer {
    pub fn new(id: PeerId, address: Multiaddr) -> Self {
        Self { id, address }
    }
}
