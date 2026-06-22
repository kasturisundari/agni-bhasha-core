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

/// Publish a local package to the decentralized network (IPFS/Akasha simulation)
pub async fn publish_package(file_path: &str) {
    println!("ॐ Publishing package (Sangraha) to Akasha Network: {}", file_path);
    
    match fs::read_to_string(file_path).await {
        Ok(content) => {
            // Simulate IPFS CID hashing
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            let cid = format!("Qm{}", hex::encode(hasher.finalize())[..44].to_string());
            
            println!("✅ Package Published Successfully!");
            println!("🔗 Akasha CID: {}", cid);
            println!("📦 Command to install globally:");
            println!("   kasturisundari sangraha install akasha://{}", cid);
        }
        Err(e) => eprintln!("✗ स्थानीयसङ्कुलस्य पठने त्रुटिः: {}", e),
    }
}
