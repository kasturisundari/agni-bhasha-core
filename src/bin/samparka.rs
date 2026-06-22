use kasturisundari::rpc::server::start_samparka_server;

#[tokio::main]
async fn main() {
    println!("ॐ KasturiChain Samparka RPC Node Starting...");
    // Run the JSON-RPC server on port 8545
    start_samparka_server(8545).await;
}
