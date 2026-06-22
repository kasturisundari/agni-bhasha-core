use libp2p::{
    gossipsub, kad, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId, Swarm,
};
use libp2p::core::upgrade::Version;
use libp2p::identity::Keypair;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::io;

#[derive(NetworkBehaviour)]
pub struct KasturiBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

/// --- PHASE 15: LIBP2P GOSSIP NETWORK ---
/// Initializes the LibP2P Swarm for decentralized Node discovery and transaction gossip.
pub async fn setup_p2p_swarm() -> Result<Swarm<KasturiBehaviour>, Box<dyn std::error::Error + Send + Sync>> {
    println!("🌐 Bootstrapping LibP2P Global Gossip Network...");
    
    // Create a random PeerId
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("🔑 Local Peer ID: {}", local_peer_id);

    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) 
        .validation_mode(gossipsub::ValidationMode::Strict) 
        .message_id_fn(message_id_fn) 
        .build()
        .expect("Valid config");

    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    ).expect("Correct configuration");

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("kasturi-mainnet-blocks");
    gossipsub.subscribe(&topic)?;

    // Create a Swarm to manage peers and events
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            let store = kad::store::MemoryStore::new(key.public().to_peer_id());
            let kademlia = kad::Behaviour::new(key.public().to_peer_id(), store);
            Ok(KasturiBehaviour { gossipsub, mdns, kademlia })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Listen on fixed port
    swarm.listen_on("/ip4/0.0.0.0/tcp/10809".parse()?)?;

    // Dial the Genesis Bootnode (unless we are the bootnode)
    let bootnode_addr: Multiaddr = "/ip4/52.72.236.75/tcp/10809".parse()?;
    
    // Only dial if we are NOT the genesis server
    // (We can check if our local IP is 52.72.236.75, or just blindly dial and ignore the error)
    if let Err(e) = swarm.dial(bootnode_addr) {
        println!("   ℹ️ Could not dial Genesis Bootnode (We might be the Genesis Node!): {:?}", e);
    } else {
        println!("   🌍 Dialing Genesis Bootnode at 52.72.236.75...");
    }

    Ok(swarm)
}
