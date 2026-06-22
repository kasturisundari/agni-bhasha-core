// Sanchar Protocol (सञ्चार) - KasturiChain P2P Gossip Network
// Handles TCP Peer Discovery, Handshakes, and Block/Mempool Gossip.

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

pub struct SancharNode {
    pub port: u16,
    pub peers: Arc<Mutex<HashSet<SocketAddr>>>,
}

impl SancharNode {
    pub fn new(port: u16) -> Self {
        SancharNode {
            port,
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Start the TCP Listener to accept incoming peer connections
    pub fn start_listening(&self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).expect("Failed to bind Sanchar Port");
        println!("🌐 Sanchar P2P Node Listening on port {}...", self.port);

        let peers_clone = Arc::clone(&self.peers);

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let peer_addr = stream.peer_addr().unwrap();
                        println!("🤝 New Peer Connected: {}", peer_addr);
                        peers_clone.lock().unwrap().insert(peer_addr);
                        
                        // Handle the peer in a new thread
                        thread::spawn(move || {
                            SancharNode::handle_connection(stream);
                        });
                    }
                    Err(e) => {
                        println!("❌ Sanchar Connection Error: {}", e);
                    }
                }
            }
        });
    }

    /// Connect to a known bootnode to join the network
    pub fn connect_to_peer(&self, address: &str) {
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                println!("✅ Successfully connected to peer: {}", address);
                if let Ok(peer_addr) = stream.peer_addr() {
                    self.peers.lock().unwrap().insert(peer_addr);
                }

                // Send Handshake
                let handshake = b"KASTURI_HANDSHAKE_V1";
                stream.write_all(handshake).unwrap();
                
                thread::spawn(move || {
                    SancharNode::handle_connection(stream);
                });
            }
            Err(e) => {
                println!("⚠️ Failed to connect to peer {}: {}", address, e);
            }
        }
    }

    /// Broadcast a new block or mempool transaction to all connected peers
    pub fn broadcast_message(&self, message: &str) {
        let peers = self.peers.lock().unwrap();
        for peer in peers.iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let payload = format!("GOSSIP:{}", message);
                let _ = stream.write_all(payload.as_bytes());
            }
        }
        println!("📡 Broadcasted message to {} peers.", peers.len());
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let msg = String::from_utf8_lossy(&buffer[0..size]);
                    println!("📥 [P2P GOSSIP RCVD]: {}", msg);
                    
                    // Logic to add to local Mempool or Block DAG goes here
                }
                Ok(_) | Err(_) => {
                    println!("🔌 Peer disconnected.");
                    break;
                }
            }
        }
    }
}
