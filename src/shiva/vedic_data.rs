/// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
///  वैदिक आँकड़ा — संस्कृत विशुद्ध (शून्य आंग्ल)
/// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Nakshatra {
    pub index: usize,
    pub sanskrit: &'static str,
    pub start_deg: f64,
    pub end_deg: f64,
    pub adhipati: &'static str,
    pub devata: &'static str,
    pub rashi: &'static str,
}

pub const NAKSHATRAS: [Nakshatra; 27] = [
    Nakshatra { index: 0,  sanskrit: "अश्विनी",        start_deg: 0.0,      end_deg: 13.3333,  adhipati: "केतु",   devata: "अश्विनीकुमार", rashi: "मेष" },
    Nakshatra { index: 1,  sanskrit: "भरणी",           start_deg: 13.3333,  end_deg: 26.6667,  adhipati: "शुक्र",  devata: "यम",          rashi: "मेष" },
    Nakshatra { index: 2,  sanskrit: "कृत्तिका",       start_deg: 26.6667,  end_deg: 40.0,     adhipati: "सूर्य",  devata: "अग्नि",       rashi: "मेष/वृषभ" },
    Nakshatra { index: 3,  sanskrit: "रोहिणी",         start_deg: 40.0,     end_deg: 53.3333,  adhipati: "चन्द्र", devata: "ब्रह्मा",      rashi: "वृषभ" },
    Nakshatra { index: 4,  sanskrit: "मृगशिरा",        start_deg: 53.3333,  end_deg: 66.6667,  adhipati: "मंगल",   devata: "सोम",         rashi: "वृषभ/मिथुन" },
    Nakshatra { index: 5,  sanskrit: "आर्द्रा",         start_deg: 66.6667,  end_deg: 80.0,     adhipati: "राहु",   devata: "रुद्र",        rashi: "मिथुन" },
    Nakshatra { index: 6,  sanskrit: "पुनर्वसु",        start_deg: 80.0,     end_deg: 93.3333,  adhipati: "गुरु",   devata: "अदिति",       rashi: "मिथुन/कर्क" },
    Nakshatra { index: 7,  sanskrit: "पुष्य",          start_deg: 93.3333,  end_deg: 106.6667, adhipati: "शनि",   devata: "बृहस्पति",     rashi: "कर्क" },
    Nakshatra { index: 8,  sanskrit: "आश्लेषा",        start_deg: 106.6667, end_deg: 120.0,    adhipati: "बुध",   devata: "सर्प",         rashi: "कर्क" },
    Nakshatra { index: 9,  sanskrit: "मघा",            start_deg: 120.0,    end_deg: 133.3333, adhipati: "केतु",   devata: "पितृ",         rashi: "सिंह" },
    Nakshatra { index: 10, sanskrit: "पूर्वा फाल्गुनी",  start_deg: 133.3333, end_deg: 146.6667, adhipati: "शुक्र",  devata: "भग",          rashi: "सिंह" },
    Nakshatra { index: 11, sanskrit: "उत्तरा फाल्गुनी", start_deg: 146.6667, end_deg: 160.0,    adhipati: "सूर्य",  devata: "अर्यमा",      rashi: "सिंह/कन्या" },
    Nakshatra { index: 12, sanskrit: "हस्त",           start_deg: 160.0,    end_deg: 173.3333, adhipati: "चन्द्र", devata: "सवितृ",        rashi: "कन्या" },
    Nakshatra { index: 13, sanskrit: "चित्रा",          start_deg: 173.3333, end_deg: 186.6667, adhipati: "मंगल",   devata: "त्वष्टा",      rashi: "कन्या/तुला" },
    Nakshatra { index: 14, sanskrit: "स्वाती",          start_deg: 186.6667, end_deg: 200.0,    adhipati: "राहु",   devata: "वायु",         rashi: "तुला" },
    Nakshatra { index: 15, sanskrit: "विशाखा",         start_deg: 200.0,    end_deg: 213.3333, adhipati: "गुरु",   devata: "इन्द्राग्नि",   rashi: "तुला/वृश्चिक" },
    Nakshatra { index: 16, sanskrit: "अनुराधा",        start_deg: 213.3333, end_deg: 226.6667, adhipati: "शनि",   devata: "मित्र",        rashi: "वृश्चिक" },
    Nakshatra { index: 17, sanskrit: "ज्येष्ठा",        start_deg: 226.6667, end_deg: 240.0,    adhipati: "बुध",   devata: "इन्द्र",       rashi: "वृश्चिक" },
    Nakshatra { index: 18, sanskrit: "मूल",            start_deg: 240.0,    end_deg: 253.3333, adhipati: "केतु",   devata: "निर्ऋति",      rashi: "धनु" },
    Nakshatra { index: 19, sanskrit: "पूर्वा आषाढ़ा",    start_deg: 253.3333, end_deg: 266.6667, adhipati: "शुक्र",  devata: "आपः",         rashi: "धनु" },
    Nakshatra { index: 20, sanskrit: "उत्तरा आषाढ़ा",   start_deg: 266.6667, end_deg: 280.0,    adhipati: "सूर्य",  devata: "विश्वेदेव",    rashi: "धनु/मकर" },
    Nakshatra { index: 21, sanskrit: "श्रवण",          start_deg: 280.0,    end_deg: 293.3333, adhipati: "चन्द्र", devata: "विष्णु",       rashi: "मकर" },
    Nakshatra { index: 22, sanskrit: "धनिष्ठा",        start_deg: 293.3333, end_deg: 306.6667, adhipati: "मंगल",   devata: "वसु",          rashi: "मकर/कुम्भ" },
    Nakshatra { index: 23, sanskrit: "शतभिषा",         start_deg: 306.6667, end_deg: 320.0,    adhipati: "राहु",   devata: "वरुण",         rashi: "कुम्भ" },
    Nakshatra { index: 24, sanskrit: "पूर्वा भाद्रपद",   start_deg: 320.0,    end_deg: 333.3333, adhipati: "गुरु",   devata: "अजैकपाद",     rashi: "कुम्भ/मीन" },
    Nakshatra { index: 25, sanskrit: "उत्तरा भाद्रपद",  start_deg: 333.3333, end_deg: 346.6667, adhipati: "शनि",   devata: "अहिर्बुध्न्य",  rashi: "मीन" },
    Nakshatra { index: 26, sanskrit: "रेवती",          start_deg: 346.6667, end_deg: 360.0,    adhipati: "बुध",   devata: "पूषा",         rashi: "मीन" },
];

pub const TITHI_NAMES: [&str; 30] = [
    "प्रतिपदा", "द्वितीया", "तृतीया", "चतुर्थी", "पंचमी",
    "षष्ठी", "सप्तमी", "अष्टमी", "नवमी", "दशमी",
    "एकादशी", "द्वादशी", "त्रयोदशी", "चतुर्दशी", "पूर्णिमा",
    "प्रतिपदा", "द्वितीया", "तृतीया", "चतुर्थी", "पंचमी",
    "षष्ठी", "सप्तमी", "अष्टमी", "नवमी", "दशमी",
    "एकादशी", "द्वादशी", "त्रयोदशी", "चतुर्दशी", "अमावस्या",
];

pub const PAKSHA_NAMES: [&str; 2] = ["शुक्ल पक्ष", "कृष्ण पक्ष"];

pub const VEDIC_MONTHS: [&str; 12] = [
    "चैत्र", "वैशाख", "ज्येष्ठ", "आषाढ़", "श्रावण", "भाद्रपद",
    "आश्विन", "कार्तिक", "मार्गशीर्ष", "पौष", "माघ", "फाल्गुन",
];

pub const VARAS: [&str; 7] = [
    "रविवार", "सोमवार", "मंगलवार", "बुधवार", "गुरुवार", "शुक्रवार", "शनिवार",
];

pub const RASHIS: [&str; 12] = [
    "मेष", "वृषभ", "मिथुन", "कर्क", "सिंह", "कन्या",
    "तुला", "वृश्चिक", "धनु", "मकर", "कुम्भ", "मीन",
];

pub const YOGA_NAMES: [&str; 27] = [
    "विष्कम्भ", "प्रीति", "आयुष्मान", "सौभाग्य", "शोभन", "अतिगण्ड",
    "सुकर्मा", "धृति", "शूल", "गण्ड", "वृद्धि", "ध्रुव",
    "व्याघात", "हर्षण", "वज्र", "सिद्धि", "व्यतीपात", "वरीयान",
    "परिघ", "शिव", "सिद्ध", "साध्य", "शुभ", "शुक्ल",
    "ब्रह्म", "इन्द्र", "वैधृति",
];

pub const KARANA_REPEATING: [&str; 7] = [
    "बव", "बालव", "कौलव", "तैतिल", "गरज", "वणिज", "विष्टि",
];

pub const KARANA_FIXED: [&str; 4] = [
    "किंस्तुघ्न", "शकुनि", "चतुष्पद", "नागव",
];

pub const GRAHA_NAMES: [&str; 9] = [
    "सूर्य", "चन्द्र", "मंगल", "बुध", "गुरु", "शुक्र", "शनि", "राहु", "केतु",
];

pub const GRAHA_SYMBOLS: [&str; 9] = [
    "☉", "☽", "♂", "☿", "♃", "♀", "♄", "☊", "☋",
];

// ─── स्थिरांक ─────────────────────────────────────────────
pub const NAKSHATRA_SPAN: f64 = 13.333333333;
pub const TITHI_SPAN: f64 = 12.0;
pub const MATHURA_LON: f64 = 77.67;
pub const MATHURA_LAT: f64 = 27.49;
pub const TROPICAL_YEAR: f64 = 365.24219;
pub const SYNODIC_MONTH: f64 = 29.530588853;
