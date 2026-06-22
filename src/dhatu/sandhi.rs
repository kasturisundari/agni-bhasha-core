/// #    (Sandhi)
///
///      (Dhatu)   (Pratyaya)
///     (Sandhi/Inflection).
///
/// कस्तूरीसुन्दरी

///      
pub fn apply_sandhi(root: &str, suffix: &str) -> String {
    match (root, suffix) {
        // ═══    (सृष्टि) ═══
        ("sṛj", "ति") => "सृजति".to_string(),     // srijati
        ("bhū", "ति") => "भवति".to_string(),      // bhavati
        ("jan", "ति") => "जायते".to_string(),     // jayate
        ("kḷp", "ति") => "कल्पते".to_string(),    // kalpate

        // ═══    (स्थिति) ═══
        ("dhā", "ति") => "दधाति".to_string(),     // dadhati
        ("sthā", "ति") => "तिष्ठति".to_string(),  // tishthati
        ("grah", "ति") => "गृह्णाति".to_string(), // grihnati

        // ═══    (ज्ञान) ═══
        ("vid", "ति") => "वेत्ति".to_string(),    // vetti
        ("jñā", "ति") => "जानाति".to_string(),    // janati
        ("dṛś", "ति") => "पश्यति".to_string(),    // pashyati
        ("cint", "ति") => "चिन्तयति".to_string(), // chintayati

        // ═══    (गति) ═══
        ("gam", "ति") => "गच्छति".to_string(),    // gacchati
        ("car", "ति") => "चरति".to_string(),      // charati
        ("sṛ", "ति") => "सरति".to_string(),       // sarati
        ("prāp", "ति") => "प्राप्नोति".to_string(), // prapnoti

        // ═══    (सम्बन्ध) ═══
        ("vac", "ति") => "वक्ति".to_string(),     // vakti
        ("śru", "ति") => "शृणोति".to_string(),    // shrinoti
        ("preṣ", "ति") => "प्रेषयति".to_string(), // preshayati
        ("bandh", "ति") => "बध्नाति".to_string(), // badhnati

        // ═══    (परिवर्तन) ═══
        ("kṛ", "ति") => "करोति".to_string(),      // karoti
        ("vṛt", "ति") => "वर्तते".to_string(),    // vartate
        ("muc", "ति") => "मुञ्चति".to_string(),   // munchati

        // ═══    (रक्षा) ═══
        ("rakṣ", "ति") => "रक्षति".to_string(),   // rakshati
        ("parīkṣ", "ति") => "परीक्षते".to_string(),// parikshate
        ("pat", "ति") => "पतति".to_string(),      // patati

        // ═══    (गणना) ═══
        ("gaṇ", "ति") => "गणयति".to_string(),     // ganayati
        ("mā", "ति") => "मिमीते".to_string(),     // mimite
        ("as", "ति") => "अस्ति".to_string(),      // asti

        // ═══   ═══
        ("bhū", "स्यति") => "भविष्यति".to_string(), // bhavishyati (async)
        ("kṛ", "स्यति") => "करिष्यति".to_string(), // karishyati (async)
        ("vac", "स्यति") => "वक्ष्यति".to_string(), // vakshyati (async)
        ("gam", "स्यति") => "गमिष्यति".to_string(), // gamishyati (async)

        ("bhū", "तव्य") => "भवितव्य".to_string(), // bhavitavya (must)
        ("kṛ", "तव्य") => "कर्तव्य".to_string(),  // kartavya (must)

        ("kṛ", "क्त") => "कृत".to_string(),       // krita (completed)
        ("bhū", "क्त") => "भूत".to_string(),      // bhuta (completed)

        ("gam", "तुम्") => "गन्तुम्".to_string(), // gantum (callback)
        ("kṛ", "तुम्") => "कर्तुम्".to_string(),  // kartum (callback)

        //         ( )
        (r, s) => format!("√{}+{}", r, s),
    }
}
