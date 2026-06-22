/// #   — Builtins
///
///       ब्रजभाषा RK.
use super::environment::Value;
use std::time::Duration;
use tokio::time::sleep;

///      
pub async fn execute_builtin(root: &str, suffix: &str, params: &[Value]) -> Option<Value> {
    match root {
        // √vac
        "वच्" => Some(builtin_vac(suffix, params)),
        // √sṛj
        "सृज्" => Some(builtin_srj(params)),
        // √gaṇ
        "गण्" => Some(builtin_gan(suffix, params)),
        // √yuj
        "युज्" => Some(builtin_yuj(params)),
        // √bhid
        "भिद्" => Some(builtin_bhid(params)),
        // √mā
        "मा" => Some(builtin_ma(params)),
        // √kram
        "क्रम्" => Some(builtin_kram(params)),
        // √as
        "अस्" => Some(builtin_as(params)),
        // √car
        "चर्" => Some(builtin_car(params)),
        
        // --- ---
        
        // √paṭh
        "पठ्" => Some(builtin_path(suffix, params).await),
        
        // √likh
        "लिख्" => Some(builtin_likh(suffix, params).await),

        // √rūp
        "रूप्" => Some(builtin_rup(suffix, params)),
        
        // √kṣip
        "क्षिप्" => Some(builtin_ksip(suffix, params).await),
        
        // √kāl
        "काल" => Some(builtin_kal(suffix, params).await),
        
        // √ākāśa
        "आकाश" => Some(builtin_akasha(suffix, params).await),

        // √kṛ
        "कृ" => Some(builtin_kru(suffix, params)),

        // √jāla
        "जाल" => Some(builtin_jala(suffix, params).await),

        _ => None,
    }
}

/// √vac+ति —  / 
fn builtin_vac(suffix: &str, params: &[Value]) -> Value {
    for param in params {
        print!("{}", param);
    }
    match suffix {
        "णम्" => {} // event —   
        _ => println!(), // ति —   
    }
    Value::Shunya
}

/// √kṛ — Factory Pattern (Unadi Tokenization)
fn builtin_kru(suffix: &str, params: &[Value]) -> Value {
    let unadi = crate::vyakarana::UnadiEngine::new();
    
    let action_str = if !params.is_empty() {
        format!("{}", params[0])
    } else {
        "कृ".to_string()
    };
    
    let val = if params.len() > 1 {
        if let Value::Integer(n) = params[1] { n as u64 } else { 1 }
    } else {
        1
    };

    let asset = unadi.derive_asset(&action_str, val);
    Value::Asset(asset)
}

/// √sṛj+ति —  
fn builtin_srj(params: &[Value]) -> Value {
    params.first().cloned().unwrap_or(Value::Shunya)
}

/// √gaṇ —  
fn builtin_gan(suffix: &str, params: &[Value]) -> Value {
    match suffix {
        "ति" => {
            if params.len() >= 2 {
                match (&params[0], &params[1]) {
                    (Value::Integer(a), Value::Integer(b)) => Value::Integer(urdhva_tiryagbhyam(*a, *b)),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                    (Value::Integer(a), Value::Float(b)) => Value::Float(*a as f64 * b),
                    (Value::Float(a), Value::Integer(b)) => Value::Float(a * *b as f64),
                    (Value::Str(op), Value::Float(val)) => {
                        match op.as_str() {
                            "sin" => Value::Float(val.sin()),
                            "cos" => Value::Float(val.cos()),
                            "tan" => Value::Float(val.tan()),
                            "sqrt" => Value::Float(val.sqrt()),
                            "round" => Value::Float(val.round()),
                            "floor" => Value::Float(val.floor()),
                            "ceil" => Value::Float(val.ceil()),
                            _ => Value::Shunya,
                        }
                    }
                    (Value::Str(op), Value::Integer(val)) => {
                        let fval = *val as f64;
                        match op.as_str() {
                            "sin" => Value::Float(fval.sin()),
                            "cos" => Value::Float(fval.cos()),
                            "tan" => Value::Float(fval.tan()),
                            "sqrt" => Value::Float(fval.sqrt()),
                            _ => Value::Shunya,
                        }
                    }
                    _ => Value::Shunya,
                }
            } else {
                params.first().cloned().unwrap_or(Value::Shunya)
            }
        }
        "आः" => {
            let mut sum = 0i64;
            for p in params {
                if let Value::Integer(n) = p { sum += n; }
            }
            Value::Integer(sum)
        }
        _ => params.first().cloned().unwrap_or(Value::Shunya),
    }
}

/// √yuj — 
fn builtin_yuj(params: &[Value]) -> Value {
    if params.len() >= 2 {
        match (&params[0], &params[1]) {
            (Value::Str(a), Value::Str(b)) => Value::Str(format!("{}{}", a, b)),
            (Value::List(a), Value::List(b)) => {
                let mut merged = a.clone();
                merged.extend(b.iter().cloned());
                Value::List(merged)
            }
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            _ => Value::Shunya,
        }
    } else {
        Value::Shunya
    }
}

/// √bhid — 
fn builtin_bhid(params: &[Value]) -> Value {
    if let Some(Value::Str(s)) = params.first() {
        if let Some(Value::Str(sep)) = params.get(1) {
            Value::List(s.split(sep.as_str()).map(|p| Value::Str(p.to_string())).collect())
        } else {
            Value::List(s.chars().map(|c| Value::Str(c.to_string())).collect())
        }
    } else {
        Value::Shunya
    }
}

/// √mā —  
fn builtin_ma(params: &[Value]) -> Value {
    match params.first() {
        Some(Value::Str(s)) => Value::Integer(s.len() as i64),
        Some(Value::List(l)) => Value::Integer(l.len() as i64),
        Some(Value::Dict(d)) => Value::Integer(d.len() as i64),
        _ => Value::Integer(0),
    }
}

/// √kram — 
fn builtin_kram(params: &[Value]) -> Value {
    if let Some(Value::List(items)) = params.first() {
        let mut sorted = items.clone();
        sorted.sort_by(|a, b| {
            match (a, b) {
                (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
                (Value::Str(x), Value::Str(y)) => x.cmp(y),
                _ => std::cmp::Ordering::Equal,
            }
        });
        Value::List(sorted)
    } else {
        Value::Shunya
    }
}

/// √as —   / 
fn builtin_as(params: &[Value]) -> Value {
    match params.first() {
        Some(val) => Value::Str(val.type_name().to_string()),
        None => Value::Str("शून्य".to_string()),
    }
}

/// √car —  (  )
fn builtin_car(params: &[Value]) -> Value {
    match (params.first(), params.get(1)) {
        (Some(Value::Integer(start)), Some(Value::Integer(end))) => {
            // --- PHASE 12 PATCH: Range Bomb Protection in √car ---
            let range_size = if *end >= *start { (*end - *start + 1) as usize } else { 0 };
            if range_size > 100_000 {
                eprintln!("🛑 √car range too large: {} elements. Max 100,000.", range_size);
                return Value::List(Vec::new());
            }
            let range: Vec<Value> = (*start..=*end).map(Value::Integer).collect();
            Value::List(range)
        }
        _ => Value::List(Vec::new()),
    }
}

/// √paṭh —    
async fn builtin_path(_suffix: &str, params: &[Value]) -> Value {
    if let Some(Value::Str(path)) = params.first() {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => Value::Str(content),
            Err(e) => {
                eprintln!("✗ त्रुटिः (पठ्): {}", e);
                Value::Shunya
            }
        }
    } else {
        Value::Shunya
    }
}

/// √kṣip —  HTTP  
async fn builtin_ksip(_suffix: &str, params: &[Value]) -> Value {
    if let Some(Value::Str(url)) = params.first() {
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(text) = resp.text().await {
                    Value::Str(text)
                } else {
                    Value::Shunya
                }
            }
            Err(e) => {
                eprintln!("✗ त्रुटिः (क्षिप्): {}", e);
                Value::Shunya
            }
        }
    } else {
        Value::Shunya
    }
}

/// √kāl —   /   
async fn builtin_kal(suffix: &str, params: &[Value]) -> Value {
    match suffix {
        "नक्षत्र" => {
            let jd = crate::shiva::system_time_to_jd();
            let panchang = crate::shiva::get_full_panchang(jd);
            Value::Str(panchang.nakshatra)
        }
        "तिथि" => {
            let jd = crate::shiva::system_time_to_jd();
            let panchang = crate::shiva::get_full_panchang(jd);
            Value::Str(panchang.tithi)
        }
        _ => {
            if let Some(Value::Integer(ms)) = params.first() {
                // --- PHASE 12 PATCH: Sleep Bomb Protection ---
                // Cap sleep to 10 seconds to prevent executor thread hijacking
                let max_sleep_ms = 10_000;
                let safe_ms = (*ms as u64).min(max_sleep_ms);
                if (*ms as u64) > max_sleep_ms {
                    eprintln!("🛑 √kāl sleep capped from {}ms to {}ms", ms, max_sleep_ms);
                }
                sleep(Duration::from_millis(safe_ms)).await;
                Value::Tattva(crate::evaluator::TattvaState::Sat)
            } else {
                Value::Shunya
            }
        }
    }
}

/// √ākāśa —    (Content-Addressable Storage)
async fn builtin_akasha(suffix: &str, params: &[Value]) -> Value {
    match suffix {
        "ति" => {
            //   (Store)
            if let Some(val) = params.first() {
                let mut db = crate::storage::akasha::AKASHA_DB.lock().unwrap();
                let cid = db.store(val);
                Value::Str(cid)
            } else {
                Value::Shunya
            }
        }
        "णम्" => {
            //   (Retrieve)
            if let Some(Value::Str(cid)) = params.first() {
                let db = crate::storage::akasha::AKASHA_DB.lock().unwrap();
                if let Some(val) = db.retrieve(cid) {
                    val
                } else {
                    Value::Shunya
                }
            } else {
                Value::Shunya
            }
        }
        _ => Value::Shunya,
    }
}

///    ( )
fn urdhva_tiryagbhyam(mut a: i64, mut b: i64) -> i64 {
    let sign = if (a < 0) ^ (b < 0) { -1 } else { 1 };
    a = a.abs();
    b = b.abs();
    
    //   (Vertically and Crosswise)
    let a_hi = a >> 32;
    let a_lo = a & 0xFFFFFFFF;
    let b_hi = b >> 32;
    let b_lo = b & 0xFFFFFFFF;
    
    let v1 = a_lo.wrapping_mul(b_lo);
    let c1 = a_hi.wrapping_mul(b_lo);
    let c2 = a_lo.wrapping_mul(b_hi);
    
    let cross_sum = c1.wrapping_add(c2);
    let result = v1.wrapping_add(cross_sum << 32);
    
    result * sign
}

/// √likh —    
async fn builtin_likh(_suffix: &str, params: &[Value]) -> Value {
    if let (Some(Value::Str(path)), Some(Value::Str(content))) = (params.first(), params.get(1)) {
        match tokio::fs::write(path, content).await {
            Ok(_) => Value::Tattva(crate::evaluator::TattvaState::Sat),
            Err(e) => {
                eprintln!("✗ त्रुटिः (लिख्): {}", e);
                Value::Shunya
            }
        }
    } else {
        Value::Shunya
    }
}

///    Value  serde_json::Value
pub fn value_to_json(val: &Value) -> serde_json::Value {
    match val {
        Value::Integer(n) => serde_json::Value::Number(serde_json::Number::from(*n)),
        Value::Float(f) => serde_json::Number::from_f64(*f).map(serde_json::Value::Number).unwrap_or(serde_json::Value::Null),
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Tattva(t) => serde_json::Value::Bool(*t == crate::evaluator::TattvaState::Sat || *t == crate::evaluator::TattvaState::Sadasat),
        Value::List(l) => serde_json::Value::Array(l.iter().map(value_to_json).collect()),
        Value::Dict(d) => {
            let mut map = serde_json::Map::new();
            for (k, v) in d {
                map.insert(k.clone(), value_to_json(v));
            }
            serde_json::Value::Object(map)
        }
        Value::StructInstance { fields, .. } => {
            let mut map = serde_json::Map::new();
            for (k, v) in fields {
                map.insert(k.clone(), value_to_json(v));
            }
            serde_json::Value::Object(map)
        }
        _ => serde_json::Value::Null,
    }
}

pub fn json_to_value(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Shunya,
        serde_json::Value::Bool(b) => Value::Tattva(if *b { crate::evaluator::TattvaState::Sat } else { crate::evaluator::TattvaState::Asat }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Shunya
            }
        }
        serde_json::Value::String(s) => Value::Str(s.clone()),
        serde_json::Value::Array(a) => Value::List(a.iter().map(json_to_value).collect()),
        serde_json::Value::Object(o) => {
            let mut dict = std::collections::HashMap::new();
            for (k, v) in o {
                dict.insert(k.clone(), json_to_value(v));
            }
            Value::Dict(dict)
        }
    }
}

/// √rūp —  (Serialization/Deserialization)  JSON
fn builtin_rup(suffix: &str, params: &[Value]) -> Value {
    if let Some(val) = params.first() {
        match suffix {
            "ति" => {
                // Parse JSON string to Value
                if let Value::Str(s) = val {
                    match serde_json::from_str(s) {
                        Ok(json) => json_to_value(&json),
                        Err(_) => Value::Shunya,
                    }
                } else {
                    Value::Shunya
                }
            }
            "णम्" => {
                // Serialize Value to JSON string
                let json = value_to_json(val);
                match serde_json::to_string_pretty(&json) {
                    Ok(s) => Value::Str(s),
                    Err(_) => Value::Shunya,
                }
            }
            _ => Value::Shunya,
        }
    } else {
        Value::Shunya
    }
}

/// √जाल — Network Syscalls (Jāla Protocol)
async fn builtin_jala(suffix: &str, params: &[Value]) -> Value {
    use tokio::io::AsyncWriteExt;
    
    match suffix {
        "श्रु" => {
            // √जाल+श्रु (listen)
            // Example: √जाल+श्रु·("0.0.0.0:8080", "<html>Agni Bhasha Server!</html>")
            if params.len() >= 2 {
                if let (Value::Str(addr), Value::Str(content)) = (&params[0], &params[1]) {
                    println!("🌐 [Agni Syscall] √जाल+श्रु: Starting Sovereign TCP Server on {}...", addr);
                    match tokio::net::TcpListener::bind(addr).await {
                        Ok(listener) => {
                            println!("🔥 Agni Bhasha OS listening natively on {}", addr);
                            
                            // Blocking loop to handle requests natively
                            loop {
                                match listener.accept().await {
                                    Ok((mut socket, peer)) => {
                                        println!("   -> Connected by: {}", peer);
                                        let response = format!(
                                            "HTTP/1.1 200 OK\r\n\
                                            Content-Length: {}\r\n\
                                            Content-Type: text/html; charset=utf-8\r\n\
                                            Connection: close\r\n\
                                            \r\n\
                                            {}",
                                            content.len(), content
                                        );
                                        let _ = socket.write_all(response.as_bytes()).await;
                                    }
                                    Err(e) => {
                                        eprintln!("   -> Accept failed: {}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("✗ [Agni Syscall] Error binding TCP socket: {}", e);
                            Value::Shunya
                        }
                    }
                } else {
                    Value::Shunya
                }
            } else {
                Value::Shunya
            }
        }
        _ => Value::Shunya,
    }
}
