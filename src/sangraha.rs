/// Sangraha (संग्रह) - KasturiChain Package Manager
///
/// Handles downloading and resolving third-party `.sutra` modules.

use tokio::fs;

/// Install a package from a given URL into the _sangraha directory
pub async fn install_package(url: &str) {
    println!("ॐ Fetching package (Sangraha) from: {}", url);
    let dir_name = "_sangraha";
    
    if let Err(e) = fs::create_dir_all(dir_name).await {
        eprintln!("✗ _sangraha पुटस्य निर्माणे त्रुटिः: {}", e);
        return;
    }

    let file_name = url.split('/').last().unwrap_or("lib.sutra");
    let target_path = format!("{}/{}", dir_name, file_name);

    match reqwest::get(url).await {
        Ok(resp) => {
            if let Ok(text) = resp.text().await {
                match fs::write(&target_path, text).await {
                    Ok(_) => println!("✅ Package installed successfully: {}", target_path),
                    Err(e) => eprintln!("✗ सज्जिका रक्षणे त्रुटिः: {}", e),
                }
            } else {
                eprintln!("✗ सङ्कुलस्य विषयवस्तु पठितुं न शक्यते।");
            }
        }
        Err(e) => eprintln!("✗ अवतरणं विफलम्: {}", e),
    }
}

/// Publish a local package to the decentralized network (True IPFS/Akasha)
pub async fn publish_package(file_path: &str) {
    println!("ॐ Publishing package (Sangraha) to Akasha Network: {}", file_path);
    
    match fs::read_to_string(file_path).await {
        Ok(content) => {
            // Zero Mocks: True IPFS Integration
            // We connect to a running local IPFS daemon (Kubo) or a decentralized storage gateway
            let ipfs_endpoint = "http://127.0.0.1:5001/api/v0/add";
            let client = reqwest::Client::new();
            
            let form = reqwest::multipart::Form::new()
                .part("file", reqwest::multipart::Part::bytes(content.into_bytes()).file_name(file_path.to_string()));
                
            match client.post(ipfs_endpoint).multipart(form).send().await {
                Ok(resp) if resp.status().is_success() => {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        let cid = json["Hash"].as_str().unwrap_or("Unknown");
                        println!("✅ Package Published Successfully to True IPFS!");
                        println!("🔗 Akasha CID: {}", cid);
                        println!("📦 Command to install globally:");
                        println!("   kasturisundari sangraha install akasha://{}", cid);
                    } else {
                        eprintln!("✗ Failed to parse IPFS response.");
                    }
                }
                Ok(resp) => {
                    eprintln!("✗ IPFS Daemon Error: {}. Is Kubo running?", resp.status());
                    eprintln!("  Start your node: `ipfs daemon`");
                }
                Err(e) => {
                    eprintln!("✗ Could not connect to IPFS Daemon: {}", e);
                    eprintln!("  Zero Mocks Enforced: The network expects a REAL IPFS node running on port 5001.");
                }
            }
        }
        Err(e) => eprintln!("✗ स्थानीयसङ्कुलस्य पठने त्रुटिः: {}", e),
    }
}
