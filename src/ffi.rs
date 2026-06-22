/// # Kasturichain Mobile FFI
/// 
/// Provides C-ABI bindings for Flutter mobile integration.
/// This allows the Flutter app to generate keys and sign transactions
/// natively on the device using Dilithium5 Nakshatra-Entangled Quantum Cryptography.

use crate::network::quantum::QuantumSigner;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Generate a new Dilithium5 keypair.
/// Returns a JSON string containing hex-encoded "public_key" and "private_key".
#[no_mangle]
pub extern "C" fn generate_quantum_keypair() -> *mut c_char {
    let (pk, sk) = QuantumSigner::generate_keypair();
    let json = serde_json::json!({
        "public_key": hex::encode(pk),
        "private_key": hex::encode(sk)
    });
    let c_str = CString::new(json.to_string()).unwrap();
    c_str.into_raw()
}

/// Sign data with the given hex-encoded private key and a Nakshatra timestamp.
/// Returns a hex-encoded signature.
#[no_mangle]
pub extern "C" fn sign_transaction_with_nakshatra(
    private_key_hex: *const c_char,
    data: *const c_char,
    nakshatra_timestamp: u64,
) -> *mut c_char {
    let pk_c_str = unsafe {
        assert!(!private_key_hex.is_null());
        CStr::from_ptr(private_key_hex)
    };
    let data_c_str = unsafe {
        assert!(!data.is_null());
        CStr::from_ptr(data)
    };

    let pk_str = pk_c_str.to_str().unwrap_or("");
    let data_str = data_c_str.to_str().unwrap_or("");

    if let Ok(pk_bytes) = hex::decode(pk_str) {
        if let Ok(sig) = QuantumSigner::sign(&pk_bytes, data_str.as_bytes(), nakshatra_timestamp) {
            let sig_hex = hex::encode(sig);
            return CString::new(sig_hex).unwrap().into_raw();
        }
    }

    CString::new("error_signing").unwrap().into_raw()
}

/// Free a string returned by Rust
#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    if s.is_null() { return }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
