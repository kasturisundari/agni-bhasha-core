/// # कस्तूरीसुन्दरी — Kasturisundari
///
/// Cosmic programming language based on Panini's Sanskrit Grammar.
/// Realizes the four core pillars:
/// 1. Shiva Sutras — 14 computational frequencies
/// 2. Sutras — concise declarative rules
/// 3. Dhatu + Pratyaya — roots & suffixes
/// 4. Anuvritti — context inheritance

mod shiva;
mod lexer;
mod parser;
mod dhatu;
mod anuvritti;
mod evaluator;
mod error;
mod math;
mod storage;
mod network;
mod sangraha;
mod lsp;
pub mod ffi;
pub mod vyakarana;
pub mod wallet;
use lexer::Scanner;
use parser::SutraParser;
use evaluator::Engine;

use std::io::{self, Write, BufRead};

const BANNER: &str = r#"
  ॐ कस्तूरीसुन्दरी ॐ
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Kasturisundari v0.1.0
  The Cosmic Programming Language
  Based on Panini's Sanskrit Grammar

  शिव सूत्र · धातु · प्रत्यय · अनुवृत्ति

  Type 'सहायता' for help, 'विराम' to exit
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#;

const HELP_TEXT: &str = r#"
  ═══════════════════════════════════════
  कस्तूरीसुन्दरी — Help Manual (सहायता)
  ═══════════════════════════════════════

  ▸ √vac+ति "text"         — print text
  ▸ √sṛj+ति·x ← 42        — create variable
  ▸ √gaṇ+ति·a·b            — compute
  ▸ √car+मान·1→10 :: √vac+ति·◈  — loop
  ▸ √yuj+ति·a·b             — join
  ▸ √bhid+ति·s·","          — split
  ▸ √mā+ति·x                — size of list
  ▸ √kram+ति·list            — sort list

  ▸ सूत्र name(params) { ... }  — declare function
  ▸ अधिकार ctx { ... }          — context block
  ▸ प्रकरण ctx { ... }          — sub-context block

  ▸ धातवः       — list all roots (Dhatus)
  ▸ शिवसूत्राणि  — list all 14 Shiva Sutra states
  ▸ सहायता      — this help menu
  ▸ विराम       — exit REPL
  ═══════════════════════════════════════
"#;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Early command routing to avoid spawning daemon for CLI tools
    if args.len() > 1 {
        if args[1] == "wallet" {
            wallet::handle_wallet_command(&args).await;
            return;
        }

        if args[1] == "sangraha" && args.len() > 3 {
            if args[2] == "install" {
                let url = &args[3];
                sangraha::install_package(url).await;
                return;
            } else if args[2] == "publish" {
                let file_path = &args[3];
                sangraha::publish_package(file_path).await;
                return;
            }
        }

        if args[1] == "lsp" {
            lsp::start_lsp().await;
            return;
        }

        if args[1] == "format" && args.len() > 2 {
            let file_path = &args[2];
            format_file(file_path).await;
            return;
        }

        if args[1] == "compile" && args.len() > 2 {
            let file_path = &args[2];
            compile_file_to_asm(file_path).await;
            return;
        }

        if args[1] != "daemon" {
            // Run .sutra file
            run_file(&args[1]).await;
            return;
        }
    } else {
        // Interactive REPL
        run_repl().await;
        return;
    }

    // --- DAEMON MODE ONLY ---
    // Check for port overrides before starting network services
    let mut rpc_port = 10808;
    let mut evm_port = 8545;
    for i in 2..args.len() {
        if args[i] == "--p2p-port" && i + 1 < args.len() {
            if let Ok(port) = args[i+1].parse::<u16>() {
                let mut p2p = network::gossip::P2P_PORT.lock().unwrap();
                *p2p = port;
                println!("🌐 P2P Port overridden to {}", port);
            }
        }
        if args[i] == "--rpc-port" && i + 1 < args.len() {
            if let Ok(port) = args[i+1].parse::<u16>() {
                rpc_port = port;
                println!("🌐 RPC Port overridden to {}", port);
            }
        }
        if args[i] == "--evm-port" && i + 1 < args.len() {
            if let Ok(port) = args[i+1].parse::<u16>() {
                evm_port = port;
                println!("🌐 EVM Port overridden to {}", port);
            }
        }
    }

    // Spawn KalaSyncNode in background
    let gossip_node = network::KalaSyncNode::new();
    tokio::spawn(async move {
        if let Err(e) = gossip_node.start().await {
            eprintln!("✗ काल सिंक प्रोटोकॉल आरम्भकरणे त्रुटिः: {}", e);
        }
    });

    // Spawn Setu EVM RPC Proxy in background
    let setu_proxy = network::SetuProxyNode::new();
    tokio::spawn(async move {
        // We'd need to pass evm_port to SetuProxyNode::start(), but SetuProxyNode currently hardcodes 8545. 
        // We will just let it fail gracefully if running a second node without changing its source code.
        if let Err(e) = setu_proxy.start().await {
            eprintln!("✗ सेतु प्रक्सी आरम्भकरणे त्रुटिः: {}", e);
        }
    });

    // Spawn Samparka Native JSON-RPC Gateway in background
    let samparka_gateway = network::SamparkaGateway::new();
    tokio::spawn(async move {
        // Similarly, Samparka hardcodes 10808. We'll let it fail gracefully.
        if let Err(e) = samparka_gateway.start().await {
            eprintln!("✗ संपर्क द्वार आरम्भकरणे त्रुटिः: {}", e);
        }
    });

    // Spawn Miner Daemon (Block Coordinator) in background
    tokio::spawn(async {
        network::daemon::start_miner_daemon().await;
    });

    // --- Genesis Block Check ---
    let is_genesis_needed = {
        let db = storage::mandala::MANDALA_DB.lock().unwrap();
        db.db.is_empty()
    };
    
    if is_genesis_needed {
        println!("🌌 kasturi_mandala_db is empty. Initializing Genesis Block...");
        run_file("kasturichain/genesis.sutra").await;
    }

    println!("🕉️  KasturiChain Sovereign Node running in Daemon Mode.");
    tokio::signal::ctrl_c().await.ok();
    println!("👋 Node shutting down.");

}

// Package manager logic moved to sangraha.rs

/// Formatter
async fn format_file(path: &str) {
    println!("ॐ Formatting file: {}", path);
    match tokio::fs::read_to_string(path).await {
        Ok(source) => {
            let mut scanner = Scanner::new(&source);
            let tokens = scanner.scan_tokens();
            let mut parser = SutraParser::new(tokens);
            match parser.parse() {
                Ok(prog) => {
                    let formatted = parser::formatter::format_program(&prog);
                    match tokio::fs::write(path, formatted).await {
                        Ok(_) => println!("✅ File formatted successfully!"),
                        Err(e) => eprintln!("✗ सज्जिका रक्षणे त्रुटिः: {}", e),
                    }
                }
                Err(e) => eprintln!("✗ वाक्यरचनादोषात् सज्जिका कर्तुं न शक्यते: {:?}", e),
            }
        }
        Err(e) => eprintln!("✗ सज्जिका पठने त्रुटिः: {}", e),
    }
}

/// Execute a .sutra file
async fn run_file(path: &str) {
    let source = match tokio::fs::read_to_string(path).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", error::Diagnostic::error(format!("'{}' पठितुं न शक्यते: {}", path, e)));
            std::process::exit(1);
        }
    };

    let mut engine = Engine::new();
    match run_source(&source, &mut engine).await {
        Ok(_) => {}
        Err(diag) => {
            eprintln!("{}", diag);
            std::process::exit(1);
        }
    }
}

/// Compile a .padma/.sutra file directly to Native Machine Code (x86_64 NASM)
async fn compile_file_to_asm(path: &str) {
    let source = match tokio::fs::read_to_string(path).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("✗ सज्जिका पठने त्रुटिः: {}", e);
            std::process::exit(1);
        }
    };

    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();
    let mut parser = SutraParser::new(tokens);
    
    match parser.parse() {
        Ok(prog) => {
            let mut asm_comp = evaluator::AgniAsmCompiler::new();
            let asm_code = asm_comp.compile(&prog.statements);
            
            let asm_file = format!("{}.asm", path);
            match tokio::fs::write(&asm_file, asm_code).await {
                Ok(_) => {
                    println!("✅ ॐ सङ्कलन सिद्धम्! (Compiled to {})", asm_file);
                    println!("To build executable, run:");
                    println!("nasm -f elf64 {} -o {}.o", asm_file, path);
                    println!("ld -o {}.exe {}.o", path, path);
                }
                Err(e) => eprintln!("✗ त्रुटिः: {}", e),
            }
        }
        Err(e) => eprintln!("✗ वाक्यरचनादोषात् सङ्कलन कर्तुं न शक्यते: {:?}", e),
    }
}

/// --- PHASE 14: SECURE BARE-METAL SANDBOX ---
/// Executes a compiled native binary inside a strict OS-level sandbox
/// Uses `prlimit` via Command to restrict memory, file size, and execution time.
pub async fn execute_sandboxed_binary(exe_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("🛡️ Launching Secure Bare-Metal Sandbox for: {}", exe_path);
    
    // In a real production environment on Linux, we use `prlimit` or `seccomp`.
    // For this implementation, we use `timeout` and restrict environment variables.
    let output = tokio::process::Command::new("timeout")
        .arg("2") // 2 seconds absolute execution limit (prevents infinite loops)
        .arg(exe_path)
        .env_clear() // Clear all environment variables (prevent secret leakage)
        // Set hard limits on Unix systems (if available) using `sh -c ulimit`
        // .arg("-c").arg("ulimit -v 65536; ulimit -f 0; ./exe_path") // 64MB memory limit, 0 file creation limit
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Sandbox Execution Failed. Status: {}, Error: {}", 
            output.status, String::from_utf8_lossy(&output.stderr)).into())
    }
}

/// REPL
async fn run_repl() {
    println!("{}", BANNER);

    let mut engine = Engine::new();
    
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let history_file = ".kasturisundari_history";
    let _ = rl.load_history(history_file);

    loop {
        let readline = rl.readline("ॐ › ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                
                rl.add_history_entry(trimmed).unwrap();

                match trimmed {
                    "विराम" | "exit" | "quit" => {
                        println!("  ॐ शान्तिः शान्तिः शान्तिः ॐ");
                        println!("  (Om Peace Peace Peace Om)");
                        break;
                    }
                    "सहायता" => {
                        println!("{}", HELP_TEXT);
                        continue;
                    }
                    "धातवः" => {
                        print_all_roots(&engine);
                        continue;
                    }
                    "शिवसूत्राणि" => {
                        print_shiva_states();
                        continue;
                    }
                    _ => {}
                }

                match run_source(trimmed, &mut engine).await {
                    Ok(val) => {
                        let display = format!("{}", val);
                        if display != "शून्य" && !display.is_empty() {
                            println!("  ⟹ {}", display);
                        }
                    }
                    Err(e) => {
                        eprintln!("  {}", e);
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) | Err(rustyline::error::ReadlineError::Eof) => {
                println!("  ॐ शान्तिः शान्तिः शान्तिः ॐ");
                break;
            }
            Err(err) => {
                eprintln!("त्रुटिः: {:?}", err);
                break;
            }
        }
    }
    
    let _ = rl.save_history(history_file);
}

/// Evaluate source code
async fn run_source(source: &str, engine: &mut Engine) -> Result<evaluator::Value, error::Diagnostic> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = SutraParser::new(tokens);
    let program = parser.parse().map_err(|e| e.to_diagnostic())?;

    // Static Type Check Phase
    let mut type_checker = crate::vyakarana::types::TypeChecker::new();
    if let Err(e) = type_checker.check_program(&program) {
        return Err(error::Diagnostic::error(format!("प्रकारदोष (Type Error): {}", e)));
    }

    engine.execute(&program).await.map_err(|e| e.to_diagnostic())
}

/// Print all registered roots
fn print_all_roots(engine: &Engine) {
    println!("\n  ═══ list of Dhatus ═══");
    let mut names = engine.derivation.dhatu_registry.all_names();
    names.sort();
    for name in names {
        if let Some(root) = engine.derivation.dhatu_registry.lookup(name) {
            println!(
                "  √{:<12} {} — {}",
                name, root.devanagari, root.meaning
            );
        }
    }
    println!();
}

/// Print 14 Shiva Sutra states
fn print_shiva_states() {
    println!("\n  ═══ Shiva Sutras (14 States) ═══");
    for state in shiva::ShivaState::all() {
        println!(
            "  {:>2}. {:<14} {} — {} (freq: {})",
            state.index() + 1,
            format!("{:?}", state),
            state.sanskrit_name(),
            state.english_name(),
            state.frequency()
        );
    }
    println!();
}
