use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;
use sha2::Digest;

// ---------------------------------------------------------
// JSON-RPC 2.0 Structures
// ---------------------------------------------------------

#[derive(Deserialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
    pub id: Option<Value>,
}

#[derive(Serialize, Debug)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<RpcError>,
    pub id: Option<Value>,
}

#[derive(Serialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

// ---------------------------------------------------------
// In-Memory Node State (Zero Mocks Phase 1)
// ---------------------------------------------------------

#[derive(Clone, Serialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: usize,
    pub miner: String,
    pub pyar_fee_routed: f64,
}

#[derive(Clone, Serialize, Debug)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
}

#[derive(Clone, Serialize, Debug)]
pub struct NodeState {
    pub kosha_size: u64,
    pub tps: u64,
    pub recent_blocks: Vec<Block>,
    pub mempool: Vec<Transaction>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            kosha_size: 0,
            tps: 0,
            recent_blocks: vec![],
            mempool: vec![],
        }
    }
}

pub type SharedState = Arc<Mutex<NodeState>>;

// ---------------------------------------------------------
// RPC Server Implementation
// ---------------------------------------------------------

pub async fn start_samparka_server(port: u16) {
    let state = Arc::new(Mutex::new(NodeState::new()));

    // Set up permissive CORS for Wallet and Explorer access
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/samparka", post(handle_rpc))
        .with_state(state)
        .layer(cors);

    let addr = format!("0.0.0.0:{}", port);
    println!("🕉️ Samparka RPC Gateway starting on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_rpc(
    State(state): State<SharedState>,
    Json(payload): Json<RpcRequest>,
) -> (StatusCode, Json<RpcResponse>) {
    
    // Process JSON-RPC Request
    let result = match payload.method.as_str() {
        "kosha_sthiti" => {
            let s = state.lock().unwrap();
            Ok(json!(s.clone()))
        },
        "send_transaction" => {
            // Process real signed transaction (Add to mempool)
            if let Some(tx_val) = payload.params.get(0) {
                // Simplified extraction for Zero Mocks Phase 1
                let sender = tx_val.get("sender").and_then(|v| v.as_str()).unwrap_or("0xUnknown").to_string();
                let amount = tx_val.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
                
                let tx = Transaction {
                    hash: format!("0x{}", hex::encode(sha2::Sha256::digest(tx_val.to_string().as_bytes()))).chars().take(42).collect(),
                    from: sender,
                    to: "0xReceiver".to_string(),
                    amount,
                };
                
                let mut s = state.lock().unwrap();
                s.mempool.push(tx);
                
                Ok(json!("Transaction Added to Mempool"))
            } else {
                Err(RpcError { code: -32602, message: "Invalid params".to_string() })
            }
        },
        "eth_blockNumber" => {
            let s = state.lock().unwrap();
            Ok(json!(format!("0x{:x}", s.kosha_size)))
        },
        _ => Err(RpcError {
            code: -32601,
            message: "Method not found".to_string(),
        }),
    };

    let response = match result {
        Ok(res) => RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(res),
            error: None,
            id: payload.id,
        },
        Err(err) => RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(err),
            id: payload.id,
        },
    };

    (StatusCode::OK, Json(response))
}
