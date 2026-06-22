use axum::{
    routing::post,
    Router,
    Json,
};
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;

use crate::evaluator::{Engine, RuntimeError};
use crate::parser::SutraParser;
use crate::lexer::Scanner;
use crate::shiva::nakshatra::Nakshatra;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use sha2::{Sha256, Digest};

#[derive(Debug, Deserialize)]
pub struct SamparkaRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
    pub id: Value,
}

pub struct SamparkaGateway {}

impl SamparkaGateway {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            .route("/samparka", post(handle_samparka_request))
            .layer(cors);

        let addr = SocketAddr::from(([0, 0, 0, 0], 10808));
        println!("🕉️  संपर्क द्वार (Samparka Gateway) starting on {}", addr);
        println!("📡 Native RPC URL: http://127.0.0.1:10808/samparka");

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
fn get_valid_hash(path: &str) -> Option<&'static str> {
    match path {
        "shastra/atharvaveda/book_1.json" => Some("6a0821e1b0936c4fda38c24f099215706b620bf7553224e97cdef8e37ff9f005"),
        "shastra/atharvaveda/book_10.json" => Some("079e9aeba4e7523f110deb4f3ebc923507ee67ed6b59f0ecfcbccbf59b558f49"),
        "shastra/atharvaveda/book_11.json" => Some("34b56b1eaff6240aba420362a36cdf5e8e9ae5d546fa5cc6a2dcb01ac620aa2d"),
        "shastra/atharvaveda/book_12.json" => Some("3ba7ce6f7f3a7796d98e97d3469de04ee2679ddde97bbd86b79cc396c3983cd6"),
        "shastra/atharvaveda/book_13.json" => Some("4ba62d90aea9872fbdc56bb79e3f62042cd111bd08e98ddb7bda9ccc03047ae0"),
        "shastra/atharvaveda/book_14.json" => Some("54ed261f4527b4575547c7a063a5f913b559132e16073389ddc57ca468c4d636"),
        "shastra/atharvaveda/book_15.json" => Some("2598b05ce410d3ea4e76a4e938de3b21d9bcac23d9b1b79c151246e2f918e309"),
        "shastra/atharvaveda/book_16.json" => Some("93dcffa14a34e16b5f763e68f7921f992a581a608ffe73ec701ae770d2ae763c"),
        "shastra/atharvaveda/book_17.json" => Some("52dd1ef0282908a18f2ea44488d5766d934a69d438364c15c36877ae0cc225ae"),
        "shastra/atharvaveda/book_18.json" => Some("f0bc59cac092f09db436416e2d588ad2a492841eb4fc8a14ea3797031f8a934a"),
        "shastra/atharvaveda/book_19.json" => Some("3c08c872b613bd0253030d58a6740afce5b70109f429335534d3340524750f98"),
        "shastra/atharvaveda/book_2.json" => Some("c84df08765471baf9a3cf7318883d59aec68fc91f5e7286c4b19ae884da75b80"),
        "shastra/atharvaveda/book_20.json" => Some("7ebbf372d110d1f4be95a7bd1c8fd1747b7dcc666abac72d438e7698e09e425b"),
        "shastra/atharvaveda/book_3.json" => Some("d6ca032de33c8b4e83627047ab3080594e22417c89e6ef10b2b91fb1a1e99fff"),
        "shastra/atharvaveda/book_4.json" => Some("08e0eb9d84cbc1bfd7254e10e85eb61241e32471cff05d0b5736b887991dcb3f"),
        "shastra/atharvaveda/book_5.json" => Some("573de503e6c5efb2d5000ba6891589863f8ebb2ee5bfe97d53a24dc23ee43c19"),
        "shastra/atharvaveda/book_6.json" => Some("f9cadeb9b330e572cd1d3584adcc6b9bcd30b9b0ff5d67867e1d8f16019a4e5e"),
        "shastra/atharvaveda/book_7.json" => Some("fe9128b6cb9890c19345f9be860fb8088df704120425e4c7955a5f39e3b320e2"),
        "shastra/atharvaveda/book_8.json" => Some("4470aa646cc13fa737e9f50e0387b009cfdfecaf3f06d45be5a335217e758c41"),
        "shastra/atharvaveda/book_9.json" => Some("7f964f84b30ecd789e619172707561c73acd5aa1d91b31ee696980ac670d113f"),
        "shastra/bhagavatam/bhagavatam.json" => Some("0d114c794b0c78b012e7d344470abfc561d983fa5fc51865094a02b6d18ab381"),
        "shastra/brahma_samhita/brahma.json" => Some("b5df0c0d97991e33b09b829be6138e85063e08f5a9b3568ce09d476f2d2418bc"),
        "shastra/caitanya_caritamrta/cc.json" => Some("891ca10484ec0b0957f6376d4ac6313ce7b3c3f1ebaba131b60d03fbab57f841"),
        "shastra/gita/gita.json" => Some("b96a7b2b0adde028f4d0b3a77d3d8ca821dfcca6f20393a5827acda197b5e82c"),
        "shastra/gopalatapani/gopalatapani.json" => Some("af82ddc7c5580f3654d408d2ea716ccd061f555fe4cb8a483abdd8200ab97d4a"),
        "shastra/isha_upanishad/isha.json" => Some("fca4af30e5dd57df01d976ec7f26e47be6619b5c5a8116934732d849632dc979"),
        "shastra/krishnayajurveda/krishnayajurveda.json" => Some("ada7dcffcb37107da6e041d308524ba6533dcd99845adf6b913d28827fadab72"),
        "shastra/mahabharata/book_1.json" => Some("072e05fc2a97b1b351f4149c0a1b29cc0c0b9d42a771052b581149422da569a8"),
        "shastra/mahabharata/book_10.json" => Some("42ae566b9ec986859042fe5ff5bdd4e05c011b0d309a22acea129bbfa3c04eb8"),
        "shastra/mahabharata/book_11.json" => Some("247be6da44f8a25c8b77fd1dac7c39a695131ba56fec0c92b630936f3ba4d5c7"),
        "shastra/mahabharata/book_12.json" => Some("a30d313f9c4f20de8ea8c1cedf10f031598974ebfb6262ffaa3f6a3ca5a3a1a6"),
        "shastra/mahabharata/book_13.json" => Some("d020da3c4aabae28c01ead245725c34b77e5233d383d097e2227cc8b5443af9a"),
        "shastra/mahabharata/book_14.json" => Some("4ade9cd6204c156e7403c0f8f37249c61e6fa084270dd8083f2fb8a2061cc59f"),
        "shastra/mahabharata/book_15.json" => Some("7f7df0838e1681f1228a65f8e2ac7212143231ffc98148b71cdd14608f66cb46"),
        "shastra/mahabharata/book_16.json" => Some("21ea0b133de0ff197f6bc9fda761834713adb833fcd28cd67502a737c3a47a7f"),
        "shastra/mahabharata/book_17.json" => Some("dc2ac04ded23c27c75dfd1a0d7ce49276fd94c45621092ca6499406d62fbaebc"),
        "shastra/mahabharata/book_18.json" => Some("3723f394f344bc5ed30a17a5f9274c5acede695d74ec07d7aad500e1d662a36b"),
        "shastra/mahabharata/book_2.json" => Some("c439c59ef0a0246553b723ccd2862c65e2636d3d0bc2d8b01b911b9a8964aac0"),
        "shastra/mahabharata/book_3.json" => Some("ff0cb7f2afdeba1e07d653fe86510de0b1ef40951ece06d6738011cfe3ca0d0e"),
        "shastra/mahabharata/book_4.json" => Some("06604859c08fde01a8bf4e1889cc3f715980cdd368cb314835cd16a9088905b9"),
        "shastra/mahabharata/book_5.json" => Some("29b65038c04f0caaeb76daca5b4cb775488cf6928050453687350bccae7dfadd"),
        "shastra/mahabharata/book_6.json" => Some("06673a78db84c968b6fb753f61fcb028b0209b8102a97b510058fb3ad71e33de"),
        "shastra/mahabharata/book_7.json" => Some("7223982bcb0793d9b1b06c45c396b6bc355db47b127dd4b13e4f0fe1d66bace3"),
        "shastra/mahabharata/book_8.json" => Some("8301858331de827a75feecc5df73fbba8a2ee4e8486d67dccef84ea97f735908"),
        "shastra/mahabharata/book_9.json" => Some("2653711cec397137d25306317927518db38f6f85ea33934a6a4e82aac8b9e80c"),
        "shastra/radha_sahasranama/radha.json" => Some("ba1113a364d74eb74c64f1a64939b5fc08e8b5eb0d25dcc3af54392ebb5f1782"),
        "shastra/radhika_tapani/radhika.json" => Some("3cfa86f174760df2b0da5cae9580e1ba54a1f564ac71a8e2e129cd3fa6dc4e22"),
        "shastra/ramayana/book_1.json" => Some("e1ce581e81fee1c5dad360626e03004db909960f048800905d928d05e54765cc"),
        "shastra/ramayana/book_2.json" => Some("8a6d263bb561bd71d6c2d5b68b94be2c2ca2334359daad202646065487e74d1a"),
        "shastra/ramayana/book_3.json" => Some("5ff1e384c6666bfa864e63f647dcdf69a6c6a899e641d3ff0b181d916a20e665"),
        "shastra/ramayana/book_4.json" => Some("87d24d440e5d4c3209e0479888f5585dc875396e3553220996e91d6d91998d41"),
        "shastra/ramayana/book_5.json" => Some("76690d979b96c40baedcc7ee67e696a788fa40139da82a9deb576fae6ac7cbe2"),
        "shastra/ramayana/book_6.json" => Some("d5f029e20358813f1e8eeb23516260470ed4c0071600420bef3d3b978d8a3dbc"),
        "shastra/ramayana/book_7.json" => Some("cc5548468a4f8526ecadfa406bcb4745f23538434438c9d776cffbf8126e8b26"),
        "shastra/ramcharitmanas/book_1.json" => Some("785f92c0499734c39c1bf3df76b6380313a594099163ccec87d6f8fda83c36ab"),
        "shastra/ramcharitmanas/book_2.json" => Some("08d85225e6ac9ab7371e32883cb5d381004b680bee96d4f75068d655d6ab7afa"),
        "shastra/ramcharitmanas/book_3.json" => Some("eba5f1f61672e1e8c8c0f52696be1adb053b43f2d31643eb11c627c4d3cc71f5"),
        "shastra/ramcharitmanas/book_4.json" => Some("1965dc20418e94444e3afebbbeab12e938c614a2d12c0058dde6810188e2ba13"),
        "shastra/ramcharitmanas/book_5.json" => Some("4dfee398e89f51fb187388d69db3692c6307e1178eea5eb994aefb676167ad9b"),
        "shastra/ramcharitmanas/book_6.json" => Some("241c5c71d9be53f7a9c579f2714f4f50d67790ec967ad9bbdc39bb72ce962a76"),
        "shastra/ramcharitmanas/book_7.json" => Some("ac1246f59868c2167e52b3ed61c53ce7aab74abb22a0fd3577404008b80eeec3"),
        "shastra/rigveda/book_1.json" => Some("b4e6ee4197b6ca5900e3c1322dbf9500c4b722efb903ffadcb0014138445e894"),
        "shastra/rigveda/book_10.json" => Some("f47fcdb0813bc7efe3c38aaa61517452e5ab345fdfa4d4743126f8b0f1700dfc"),
        "shastra/rigveda/book_2.json" => Some("43e90b3dc5ae65777849b34c6135d3f8df0f8dfe0f05c24d3c9fd18033d3f210"),
        "shastra/rigveda/book_3.json" => Some("d19cab40a1a69742efb5148ea37bf273d6b4c88f3e5873405b527f18b383aac0"),
        "shastra/rigveda/book_4.json" => Some("98800d8c4c3eea1e02dad807f52bec201255b3429d49e587de35ea4ad2bf6839"),
        "shastra/rigveda/book_5.json" => Some("3b3c609c34c381760306ccfea697d34d3d10d1605b7020cfbbed9fa7f1f3e624"),
        "shastra/rigveda/book_6.json" => Some("f2c43d5ebb9910e005e537caa75996da6ab9ebcbf13047fdac78336e2a23048a"),
        "shastra/rigveda/book_7.json" => Some("e80cba28dc3d6d4e6dfa859ec754c9e614ce824a45fec6316cbedd3c4b9d3497"),
        "shastra/rigveda/book_8.json" => Some("d214f98ee1c5bf99285111922b4f3bdf56cd3398abfd834764d6b328fe0679e5"),
        "shastra/rigveda/book_9.json" => Some("8af2fed05bec44b518dac2b01ced0c98c5e2960d83de1b392c47e0c12cba9465"),
        "shastra/samaveda/final-Devanagari.json" => Some("721992094cf568a47b2f3cf441fff0cb6670dc89477ff82f20fc944058a4c23d"),
        "shastra/samaveda/samaveda.json" => Some("2ff641c95d478e1b3236c8cb133cbbbef091e73f7844b24113d3210ce257a729"),
        "shastra/samaveda/updated-final-Devanagari.json" => Some("525a787fad19e1d34da5909c5d41d8c456954574239d9e1c919fe99b22c234c1"),
        "shastra/samaveda_sample.json" => Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        "shastra/upanishads/upanishads.json" => Some("0a6737ed7fd063cd3ff030693772f5bbd31ef98b8c2ccc7cf0fbc223dfb58b3f"),
        "shastra/yajurveda/yajurveda.json" => Some("ca5f1e0052ee522cf2ef131bc577cad2f5ebea256c17a5a6d0648a1db2743de1"),
        "shastra/chaitanyaupanishad/chaitanyaupanishad.json" => Some("d844ac1e2d70d0059ccd0afabcc343205f23399b5a384fe622bc1e25efbd033b"),
        "shastra/lakshmitantra/lakshmitantra.json" => Some("1acc7e21dba0ea00d276e419a500c35f87f1bad1b61b5311645f51a64dae3ad6"),
        "shastra/gautamiyatantra/gautamiyatantra.json" => Some("75639401382541e01b53a4d566864dc3ea5b5bce1586cae5d12a1ccf9a2677ee"),
        "shastra/sammohanatantra/sammohanatantra.json" => Some("712b58ad4cedf7070e558952de7433272828f2bd627748e6c768bd5e7a7d803f"),
        "shastra/naradapancaratra/naradapancaratra.json" => Some("d0289e33fe91385216dab47279f1c3a1228db36636ccdf9f39bdefb83f486aaa"),
        _ => None,
    }
}

async fn handle_samparka_request(
    Json(payload): Json<SamparkaRequest>,
) -> Json<Value> {
    let response = match payload.method.as_str() {
        "sutra_eval" => {
            if let Some(code_val) = payload.params.get(0) {
                if let Some(code) = code_val.as_str() {
                    // --- PHASE 12 PATCH: RPC Input Size Limit ---
                    // Prevent parser memory exhaustion via massive code strings
                    if code.len() > 65_536 {
                        return Json(json!({
                            "jsonrpc": "2.0",
                            "id": payload.id,
                            "error": {
                                "code": -32602,
                                "message": format!("Code too large: {} bytes exceeds 64KB limit", code.len())
                            }
                        }));
                    }
                    // --- THE DEEP DIVE PATCH #1: Remote Code Execution Prevention ---
                    // CRITICAL: RPC Engine MUST run in Sandbox mode!
                    // Without this, any internet user can read files (पठ्), write files (लिख्),
                    // and make HTTP requests (क्षिप्) on the host server.
                    let mut engine = Engine::new();
                    engine.is_sandboxed = true;
                    engine.resonance_limit = Some(5000); // Hard gas limit for RPC calls
                    let mut scanner = Scanner::new(code);
                    let tokens = scanner.scan_tokens();
                    let mut parser = SutraParser::new(tokens);
                    
                    match parser.parse() {
                        Ok(program) => {
                            match engine.execute(&program).await {
                                Ok(result) => {
                                    json!({
                                        "jsonrpc": "2.0",
                                        "id": payload.id,
                                        "result": format!("{}", result)
                                    })
                                },
                                Err(RuntimeError::ReturnValue(result)) => {
                                    // Return statements throw a ReturnValue error in our evaluator
                                    json!({
                                        "jsonrpc": "2.0",
                                        "id": payload.id,
                                        "result": format!("{}", result)
                                    })
                                },
                                Err(e) => {
                                    json!({
                                        "jsonrpc": "2.0",
                                        "id": payload.id,
                                        "error": {
                                            "code": -32000,
                                            "message": format!("Runtime Error: {}", e)
                                        }
                                    })
                                }
                            }
                        },
                        Err(e) => {
                            json!({
                                "jsonrpc": "2.0",
                                "id": payload.id,
                                "error": {
                                    "code": -32602,
                                    "message": format!("Parse Error: {:?}", e)
                                }
                            })
                        }
                    }
                } else {
                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "params[0] must be a string of Sutra code"}})
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing params for sutra_eval"}})
            }
        }
        "sutra_deploy" => {
            if let Some(code_val) = payload.params.get(0) {
                if let Some(code) = code_val.as_str() {
                    if code.len() > 262_144 {
                        return Json(json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Contract code exceeds 256KB limit"}}));
                    }
                    
                    let mut hasher = Sha256::new();
                    hasher.update(code.as_bytes());
                    let contract_hash = hex::encode(hasher.finalize());
                    let contract_addr = format!("Contract_{}", &contract_hash[0..40]);
                    
                    let mut db_lock = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                    db_lock.store(&contract_addr, &crate::evaluator::Value::Str(code.to_string()));
                    
                    json!({
                        "jsonrpc": "2.0",
                        "id": payload.id,
                        "result": contract_addr
                    })
                } else {
                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Code must be string"}})
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing code param"}})
            }
        }
        "get_history" => {
            if let Some(addr_val) = payload.params.get(0) {
                if let Some(addr) = addr_val.as_str() {
                    let history = crate::storage::indexer::INDEX_DB.get_history(addr);
                    json!({
                        "jsonrpc": "2.0",
                        "id": payload.id,
                        "result": history
                    })
                } else {
                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Address must be string"}})
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing address param"}})
            }
        }
        "get_balance" => {
            if let Some(addr_val) = payload.params.get(0) {
                if let Some(addr) = addr_val.as_str() {
                    let balance = {
                        let account_db = crate::network::account::ACCOUNT_DB.lock().unwrap();
                        account_db.get_balance(addr)
                    };
                    json!({
                        "jsonrpc": "2.0",
                        "id": payload.id,
                        "result": balance
                    })
                } else {
                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Address must be string"}})
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing address param"}})
            }
        }
        "sutra_call" => {
            if payload.params.len() >= 2 {
                if let (Some(addr_val), Some(method_val)) = (payload.params.get(0), payload.params.get(1)) {
                    if let (Some(addr), Some(method_name)) = (addr_val.as_str(), method_val.as_str()) {
                        let code_val = {
                            let db_lock = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                            db_lock.retrieve(addr)
                        };
                        
                        if let Some(crate::evaluator::Value::Str(code)) = code_val {
                            let mut engine = Engine::new();
                            engine.is_sandboxed = true;
                            engine.resonance_limit = Some(500_000); // Higher limit for contracts
                            
                            // Inject Contract Address
                            engine.env.define("यन्त्र_पता".to_string(), crate::evaluator::Value::Str(addr.to_string()));
                            
                            // Inject Context (Sender, etc) if provided in params[3]
                            if let Some(ctx_val) = payload.params.get(3) {
                                if let Some(ctx_obj) = ctx_val.as_object() {
                                    for (k, v) in ctx_obj {
                                        engine.env.define(k.clone(), crate::evaluator::builtins::json_to_value(v));
                                    }
                                }
                            }

                            let mut scanner = Scanner::new(&code);
                            let tokens = scanner.scan_tokens();
                            let mut parser = SutraParser::new(tokens);
                            
                            if let Ok(program) = parser.parse() {
                                // Load contract definition into memory
                                let _ = engine.execute(&program).await;
                                
                                // Now call the requested function
                                let mut arg_names = Vec::new();
                                if let Some(args_val) = payload.params.get(2) {
                                    if let Some(arr) = args_val.as_array() {
                                        for (i, a) in arr.iter().enumerate() {
                                            let arg_name = format!("__rpc_arg_{}", i);
                                            engine.env.define(arg_name.clone(), crate::evaluator::builtins::json_to_value(a));
                                            arg_names.push(arg_name);
                                        }
                                    }
                                }
                                
                                let call_code = format!("{}({})", method_name, arg_names.join(", "));
                                let mut call_scanner = Scanner::new(&call_code);
                                let call_tokens = call_scanner.scan_tokens();
                                let mut call_parser = SutraParser::new(call_tokens);
                                
                                if let Ok(call_prog) = call_parser.parse() {
                                    match engine.execute(&call_prog).await {
                                        Ok(res) => json!({"jsonrpc": "2.0", "id": payload.id, "result": crate::evaluator::builtins::value_to_json(&res)}),
                                        Err(RuntimeError::ReturnValue(res)) => json!({"jsonrpc": "2.0", "id": payload.id, "result": crate::evaluator::builtins::value_to_json(&res)}),
                                        Err(e) => json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32000, "message": format!("Runtime Error: {:?}", e)}})
                                    }
                                } else {
                                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Failed to parse call wrapper"}})
                                }
                            } else {
                                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Contract parse error"}})
                            }
                        } else {
                            json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Contract not found"}})
                        }
                    } else {
                        json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Params must be strings"}})
                    }
                } else {
                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Invalid params"}})
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing params for sutra_call"}})
            }
        }
        "nakshatra_kala" => {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let current_naks = Nakshatra::current_from_time(current_time);
            
            json!({
                "jsonrpc": "2.0",
                "id": payload.id,
                "result": {
                    "unix_time": current_time,
                    "nakshatra_index": current_naks as u8,
                    "nakshatra_name": current_naks.sanskrit_name()
                }
            })
        }
        "kosha_sthiti" => {
            let db_size = {
                let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                db.db.len()
            };
            json!({
                "jsonrpc": "2.0",
                "id": payload.id,
                "result": {
                    "kosha_size": db_size,
                    "status": "pavitra" // pure
                }
            })
        }
        "shringkhala_sthiti" => {
            let blocks_json = {
                let db = crate::storage::mandala::MANDALA_DB.lock().unwrap();
                if let Some(val) = db.retrieve("खण्डाः") {
                    crate::evaluator::builtins::value_to_json(&val)
                } else {
                    serde_json::Value::Null
                }
            };
            json!({
                "jsonrpc": "2.0",
                "id": payload.id,
                "result": blocks_json
            })
        }
        "shastra_jnan" => {
            if let Some(book_val) = payload.params.get(0) {
                if let Some(book_name) = book_val.as_str() {
                    let path = match book_name {
                        "gita" => "shastra/gita/gita.json".to_string(),
                        "bhagavatam" => "shastra/bhagavatam/bhagavatam.json".to_string(),
                        "caitanya" => "shastra/caitanya_caritamrta/cc.json".to_string(),
                        "brahma" => "shastra/brahma_samhita/brahma.json".to_string(),
                        "isha" => "shastra/isha_upanishad/isha.json".to_string(),
                        "radha" => "shastra/radha_sahasranama/radha.json".to_string(),
                        "radhika" => "shastra/radhika_tapani/radhika.json".to_string(),
                        "yajurveda" => "shastra/yajurveda/yajurveda.json".to_string(),
                        "samaveda" => "shastra/samaveda/samaveda.json".to_string(),
                        "krishnayajurveda" => "shastra/krishnayajurveda/krishnayajurveda.json".to_string(),
                        "gopalatapani" => "shastra/gopalatapani/gopalatapani.json".to_string(),
                        "upanishads" => "shastra/upanishads/upanishads.json".to_string(),
                        "chaitanyaupanishad" => "shastra/chaitanyaupanishad/chaitanyaupanishad.json".to_string(),
                        "lakshmitantra" => "shastra/lakshmitantra/lakshmitantra.json".to_string(),
                        "gautamiyatantra" => "shastra/gautamiyatantra/gautamiyatantra.json".to_string(),
                        "sammohanatantra" => "shastra/sammohanatantra/sammohanatantra.json".to_string(),
                        "naradapancaratra" => "shastra/naradapancaratra/naradapancaratra.json".to_string(),
                        name if name.starts_with("rigveda_") => {
                            if let Some(num) = name.strip_prefix("rigveda_") {
                                format!("shastra/rigveda/book_{}.json", num)
                            } else {
                                "".to_string()
                            }
                        },
                        name if name.starts_with("atharvaveda_") => {
                            if let Some(num) = name.strip_prefix("atharvaveda_") {
                                format!("shastra/atharvaveda/book_{}.json", num)
                            } else {
                                "".to_string()
                            }
                        },
                        name if name.starts_with("mahabharata_") => {
                            if let Some(num) = name.strip_prefix("mahabharata_") {
                                format!("shastra/mahabharata/book_{}.json", num)
                            } else {
                                "".to_string()
                            }
                        },
                        name if name.starts_with("ramcharitmanas_") => {
                            if let Some(num) = name.strip_prefix("ramcharitmanas_") {
                                format!("shastra/ramcharitmanas/book_{}.json", num)
                            } else {
                                "".to_string()
                            }
                        },
                        name if name.starts_with("ramayana_") => {
                            if let Some(num) = name.strip_prefix("ramayana_") {
                                format!("shastra/ramayana/book_{}.json", num)
                            } else {
                                "".to_string()
                            }
                        },
                        _ => "".to_string(),
                    };

                    if path.is_empty() {
                        json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Unknown Shastra"}})
                    } else {
                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                let mut hasher = Sha256::new();
                                hasher.update(content.as_bytes());
                                let hash_result = hasher.finalize();
                                let hash_hex = hex::encode(hash_result);

                                if let Some(expected_hash) = get_valid_hash(&path) {
                                    if hash_hex != expected_hash {
                                        return Json(json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32603, "message": "Tampered Shastra Detected: Hash Anchoring Failed"}}));
                                    }
                                } else {
                                    return Json(json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32603, "message": "Tampered Shastra Detected: No Hash Anchor Found"}}));
                                }

                                match serde_json::from_str::<Value>(&content) {
                                    Ok(json_data) => {
                                        json!({
                                            "jsonrpc": "2.0",
                                            "id": payload.id,
                                            "result": {
                                                "book": book_name,
                                                "hash_stamp": hash_hex,
                                                "immutable": true,
                                                "data": json_data
                                            }
                                        })
                                    },
                                    Err(e) => {
                                        json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32603, "message": format!("Failed to parse Shastra JSON: {}", e)}})
                                    }
                                }
                            },
                            Err(_) => {
                                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Shastra file not found on node"}})
                            }
                        }
                    }
                } else {
                    json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "params[0] must be book name string"}})
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing book name param"}})
            }
        }
        "send_transaction" => {
            if let Some(tx_val) = payload.params.get(0) {
                // Validate it basic structurally
                if let Err(e) = crate::network::daemon::validate_transaction(tx_val) {
                    json!({
                        "jsonrpc": "2.0",
                        "id": payload.id,
                        "error": {
                            "code": -32602,
                            "message": format!("Transaction Validation Failed: {}", e)
                        }
                    })
                } else {
                    let mut pool = crate::network::daemon::MEMPOOL.lock().unwrap();
                    
                    // --- PHASE 8 PATCH: Mempool Flooding Protection ---
                    if pool.len() >= 5000 {
                        json!({
                            "jsonrpc": "2.0",
                            "id": payload.id,
                            "error": {
                                "code": -32005,
                                "message": "Mempool is full. Try again later."
                            }
                        })
                    } else {
                        pool.push(tx_val.clone());
                        json!({
                            "jsonrpc": "2.0",
                            "id": payload.id,
                            "result": "Transaction accepted into Mempool"
                        })
                    }
                }
            } else {
                json!({"jsonrpc": "2.0", "id": payload.id, "error": {"code": -32602, "message": "Missing transaction param"}})
            }
        }
        _ => {
            json!({
                "jsonrpc": "2.0",
                "id": payload.id,
                "error": {
                    "code": -32601,
                    "message": "अमान्य विधि (Method not found in Samparka Gateway)"
                }
            })
        }
    };

    Json(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_handle_samparka_unknown_method() {
        let req = SamparkaRequest {
            jsonrpc: "2.0".to_string(),
            method: "unknown_method".to_string(),
            params: vec![],
            id: json!(1),
        };
        
        let res = handle_samparka_request(Json(req)).await;
        let val = res.0;
        
        assert_eq!(val["error"]["code"], -32601);
        assert!(val["error"]["message"].as_str().unwrap().contains("अमान्य विधि"));
    }

    #[tokio::test]
    async fn test_handle_samparka_sutra_eval_missing_params() {
        let req = SamparkaRequest {
            jsonrpc: "2.0".to_string(),
            method: "sutra_eval".to_string(),
            params: vec![],
            id: json!(1),
        };
        
        let res = handle_samparka_request(Json(req)).await;
        let val = res.0;
        
        assert_eq!(val["error"]["code"], -32602);
        assert_eq!(val["error"]["message"], "Missing code param");
    }

    #[tokio::test]
    async fn test_get_valid_hash() {
        let h1 = get_valid_hash("shastra/atharvaveda/book_1.json");
        assert!(h1.is_some());
        
        let h_invalid = get_valid_hash("nonexistent_book.json");
        assert!(h_invalid.is_none());
    }

    // A simple mock test since executing full network RPC inside tests could be complex
    #[tokio::test]
    async fn test_samparka_request_deserialization() {
        let raw = r#"{
            "jsonrpc": "2.0",
            "method": "sutra_eval",
            "params": ["माना x = 5"],
            "id": 42
        }"#;
        
        let req: SamparkaRequest = serde_json::from_str(raw).unwrap();
        assert_eq!(req.method, "sutra_eval");
        assert_eq!(req.params[0].as_str().unwrap(), "माना x = 5");
        assert_eq!(req.id.as_i64().unwrap(), 42);
    }
}
