/// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
///  ज्योतिष गणना यन्त्र — ELP2000 उच्च-परिशुद्धता
///  चन्द्र: ६०+ पद (Meeus/Chapront-Touzé)
///  सूर्य: VSOP87 सरलीकृत
/// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
use crate::shiva::vedic_data::*;
use crate::shiva::rk_epoch::julian_centuries;
use serde::Serialize;

fn deg_to_rad(d: f64) -> f64 { d * std::f64::consts::PI / 180.0 }
fn normalize360(d: f64) -> f64 { ((d % 360.0) + 360.0) % 360.0 }

// ─── अयनांश (Lahiri) ─────────────────────────────────────
pub fn get_ayanamsa(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    23.85 + (t * 100.0 * 50.2564 / 3600.0)
}

// ─── सूर्य उष्णकटिबन्धीय देशान्तर ──────────────────────
pub fn get_sun_tropical(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let l0 = normalize360(280.46646 + 36000.76983 * t + 0.0003032 * t * t);
    let m = normalize360(357.52911 + 35999.05029 * t - 0.0001537 * t * t);
    let mr = deg_to_rad(m);
    let c = (1.914602 - 0.004817 * t) * mr.sin()
          + (0.019993 - 0.000101 * t) * (2.0 * mr).sin()
          + 0.000289 * (3.0 * mr).sin();
    let sun = normalize360(l0 + c);
    let omega = 125.04 - 1934.136 * t;
    normalize360(sun - 0.00569 - 0.00478 * deg_to_rad(omega).sin())
}

// ═══════════════════════════════════════════════════════════
//  चन्द्र उष्णकटिबन्धीय देशान्तर — ELP2000 (६०+ पद)
//  Meeus "Astronomical Algorithms" सारणी ४७.अ
//  परिशुद्धता: ±0.05° (पूर्व: ±0.5°)
// ═══════════════════════════════════════════════════════════

/// ELP2000 चन्द्र देशान्तर पद: (D, M, M', F, amplitude°)
const MOON_LON_TERMS: [(i32, i32, i32, i32, f64); 60] = [
    ( 0, 0, 1, 0,  6.288774), ( 2, 0,-1, 0,  1.274027), ( 2, 0, 0, 0,  0.658314),
    ( 0, 0, 2, 0,  0.213618), ( 0, 1, 0, 0, -0.185116), ( 0, 0, 0, 2, -0.114332),
    ( 2, 0,-2, 0,  0.058793), ( 2,-1,-1, 0,  0.057066), ( 2, 0, 1, 0,  0.053322),
    ( 2,-1, 0, 0,  0.045758), ( 0, 1,-1, 0, -0.040923), ( 1, 0, 0, 0, -0.034720),
    ( 0, 1, 1, 0, -0.030383), ( 2, 0, 0,-2,  0.015327), ( 0, 0, 1, 2, -0.012528),
    ( 0, 0, 1,-2,  0.010980), ( 4, 0,-1, 0,  0.010675), ( 0, 0, 3, 0,  0.010034),
    ( 4, 0,-2, 0,  0.008548), ( 2, 1,-1, 0, -0.007888), ( 2, 1, 0, 0, -0.006766),
    ( 1, 0,-1, 0, -0.005163), ( 1, 1, 0, 0,  0.004987), ( 2,-1, 1, 0,  0.004036),
    ( 2, 0, 2, 0,  0.003994), ( 4, 0, 0, 0,  0.003861), ( 2, 0,-3, 0,  0.003665),
    ( 0, 1,-2, 0, -0.002689), ( 2, 0,-1, 2, -0.002602), ( 2,-1,-2, 0,  0.002390),
    ( 1, 0, 1, 0, -0.002348), ( 2,-2, 0, 0,  0.002236), ( 0, 1, 2, 0, -0.002120),
    ( 0, 2, 0, 0, -0.002069), ( 2,-2,-1, 0,  0.002048), ( 2, 0, 1,-2, -0.001773),
    ( 2, 0, 0, 2, -0.001595), ( 4,-1,-1, 0,  0.001215), ( 0, 0, 2, 2, -0.001110),
    ( 3, 0,-1, 0, -0.000892), ( 2, 1, 1, 0, -0.000810), ( 4,-1,-2, 0,  0.000759),
    ( 0, 2,-1, 0, -0.000713), ( 2, 2,-1, 0, -0.000700), ( 2, 1,-2, 0,  0.000691),
    ( 2,-1, 0,-2,  0.000596), ( 4, 0, 1, 0,  0.000549), ( 0, 0, 4, 0,  0.000537),
    ( 4,-1, 0, 0,  0.000520), ( 1, 0,-2, 0, -0.000487), ( 2, 1, 0,-2, -0.000399),
    ( 0, 0, 2,-2, -0.000381), ( 1, 1, 1, 0,  0.000351), ( 3, 0,-2, 0, -0.000340),
    ( 4, 0,-3, 0,  0.000330), ( 2,-1, 2, 0,  0.000327), ( 0, 2, 1, 0, -0.000323),
    ( 1, 1,-1, 0,  0.000299), ( 2, 0, 3, 0,  0.000294), ( 2, 0,-1,-2,  0.000000),
];

pub fn get_moon_tropical(jd: f64) -> f64 {
    let t = julian_centuries(jd);

    // मूल तत्त्व — उच्च-परिशुद्धता गुणांक (Meeus Ch.47)
    let lp = normalize360(
        218.3164477 + 481267.88123421 * t
        - 0.0015786 * t * t
        + t * t * t / 538841.0
        - t * t * t * t / 65194000.0
    );
    let d = normalize360(
        297.8501921 + 445267.1114034 * t
        - 0.0018819 * t * t
        + t * t * t / 545868.0
        - t * t * t * t / 113065000.0
    );
    let m = normalize360(
        357.5291092 + 35999.0502909 * t
        - 0.0001536 * t * t
        + t * t * t / 24490000.0
    );
    let mp = normalize360(
        134.9633964 + 477198.8675055 * t
        + 0.0087414 * t * t
        + t * t * t / 69699.0
        - t * t * t * t / 14712000.0
    );
    let f = normalize360(
        93.2720950 + 483202.0175233 * t
        - 0.0036539 * t * t
        - t * t * t / 3526000.0
        + t * t * t * t / 863310000.0
    );

    let dr = deg_to_rad(d);
    let mr = deg_to_rad(m);
    let mpr = deg_to_rad(mp);
    let fr = deg_to_rad(f);

    // पृथ्वी कक्षा उत्केन्द्रता संशोधन
    let e = 1.0 - 0.002516 * t - 0.0000074 * t * t;
    let e2 = e * e;

    // ६०-पद योग
    let mut lon_sum = 0.0;
    for &(dc, mc, mpc, fc, amp) in MOON_LON_TERMS.iter() {
        let arg = dc as f64 * dr + mc as f64 * mr + mpc as f64 * mpr + fc as f64 * fr;
        let mut term = amp * arg.sin();
        // M-गुणांक संशोधन (पृथ्वी उत्केन्द्रता)
        match mc.abs() {
            1 => term *= e,
            2 => term *= e2,
            _ => {}
        }
        lon_sum += term;
    }

    // अतिरिक्त संशोधन (Venus, Jupiter perturbations)
    let a1 = deg_to_rad(normalize360(119.75 + 131.849 * t));
    let a2 = deg_to_rad(normalize360(53.09 + 479264.290 * t));
    lon_sum += 0.003958 * a1.sin();
    lon_sum += 0.001962 * deg_to_rad(lp - f).sin();
    lon_sum += 0.000318 * a2.sin();

    normalize360(lp + lon_sum)
}

// ─── सायन/निरयण रूपान्तर ─────────────────────────────────
pub fn get_sidereal_sun(jd: f64) -> f64 { normalize360(get_sun_tropical(jd) - get_ayanamsa(jd)) }
pub fn get_sidereal_moon(jd: f64) -> f64 { normalize360(get_moon_tropical(jd) - get_ayanamsa(jd)) }

// ─── पञ्चाङ्ग अवयव ──────────────────────────────────────
pub fn get_nakshatra_index(sidereal: f64) -> usize { (normalize360(sidereal) / NAKSHATRA_SPAN).floor() as usize }
pub fn get_rashi_index(sidereal: f64) -> usize { (normalize360(sidereal) / 30.0).floor() as usize }
pub fn get_tithi_index(moon: f64, sun: f64) -> usize { (normalize360(moon - sun) / TITHI_SPAN).floor() as usize }
pub fn get_yoga_index(moon: f64, sun: f64) -> usize { (normalize360(moon + sun) / NAKSHATRA_SPAN).floor() as usize }
pub fn get_karana_index(moon: f64, sun: f64) -> usize { (normalize360(moon - sun) / 6.0).floor() as usize }
pub fn get_vedic_month_index(sun_sidereal: f64) -> usize { (get_rashi_index(sun_sidereal) + 1) % 12 }

pub fn get_illumination(tithi_idx: usize) -> (f64, bool) {
    if tithi_idx <= 14 {
        ((tithi_idx as f64 / 14.0) * 100.0, true)
    } else {
        (((14 - (tithi_idx.saturating_sub(15))) as f64 / 14.0) * 100.0, false)
    }
}

pub fn get_karana_name(ki: usize) -> &'static str {
    if ki == 0 { return KARANA_FIXED[0]; }
    if ki >= 57 { return KARANA_FIXED[ki - 56]; }
    KARANA_REPEATING[(ki - 1) % 7]
}

// ─── सम्पूर्ण पञ्चाङ्ग ──────────────────────────────────
#[derive(Debug, Clone, Serialize)]
pub struct FullPanchang {
    pub tithi_idx: usize,
    pub tithi: String,
    pub paksha: String,
    pub nakshatra_idx: usize,
    pub nakshatra: String,
    pub yoga_idx: usize,
    pub yoga: String,
    pub karana_idx: usize,
    pub karana: String,
    pub vara_idx: usize,
    pub vara: String,
    pub vedic_month_idx: usize,
    pub vedic_month: String,
    pub moon_sidereal: f64,
    pub sun_sidereal: f64,
    pub ayanamsa: f64,
    pub illumination_pct: f64,
    pub is_waxing: bool,
    pub elongation: f64,
    pub sun_rashi_idx: usize,
    pub sun_rashi: String,
    pub moon_rashi_idx: usize,
    pub moon_rashi: String,
    pub sun_nakshatra_idx: usize,
    pub sun_nakshatra: String,
}

pub fn get_full_panchang(jd: f64) -> FullPanchang {
    let ay = get_ayanamsa(jd);
    let ss = normalize360(get_sun_tropical(jd) - ay);
    let ms = normalize360(get_moon_tropical(jd) - ay);
    let ti = get_tithi_index(ms, ss);
    let (illum, wax) = get_illumination(ti);
    let ni = get_nakshatra_index(ms);
    let yi = get_yoga_index(ms, ss);
    let ki = get_karana_index(ms, ss);
    let vi = ((jd + 1.5).floor() as usize) % 7;
    let mi = get_vedic_month_index(ss);
    let sri = get_rashi_index(ss);
    let mri = get_rashi_index(ms);
    let sni = get_nakshatra_index(ss);

    FullPanchang {
        tithi_idx: ti,
        tithi: TITHI_NAMES[ti].to_string(),
        paksha: if ti <= 14 { PAKSHA_NAMES[0] } else { PAKSHA_NAMES[1] }.to_string(),
        nakshatra_idx: ni,
        nakshatra: NAKSHATRAS[ni].sanskrit.to_string(),
        yoga_idx: yi,
        yoga: YOGA_NAMES[yi].to_string(),
        karana_idx: ki,
        karana: get_karana_name(ki).to_string(),
        vara_idx: vi,
        vara: VARAS[vi].to_string(),
        vedic_month_idx: mi,
        vedic_month: VEDIC_MONTHS[mi].to_string(),
        moon_sidereal: ms,
        sun_sidereal: ss,
        ayanamsa: ay,
        illumination_pct: illum,
        is_waxing: wax,
        elongation: normalize360(ms - ss),
        sun_rashi_idx: sri,
        sun_rashi: RASHIS[sri].to_string(),
        moon_rashi_idx: mri,
        moon_rashi: RASHIS[mri].to_string(),
        sun_nakshatra_idx: sni,
        sun_nakshatra: NAKSHATRAS[sni].sanskrit.to_string(),
    }
}

// ─── नवग्रह गणना ─────────────────────────────────────────
#[derive(Debug, Clone, Serialize)]
pub struct GrahaPosition {
    pub sanskrit: String,
    pub symbol: String,
    pub sidereal_lon: f64,
    pub rashi_idx: usize,
    pub rashi: String,
    pub nakshatra_idx: usize,
    pub nakshatra: String,
    pub is_retrograde: bool,
}

fn make_graha(idx: usize, tropical: f64, ay: f64, retro: bool) -> GrahaPosition {
    let sid = normalize360(tropical - ay);
    let ri = get_rashi_index(sid);
    let ni = get_nakshatra_index(sid);
    GrahaPosition {
        sanskrit: GRAHA_NAMES[idx].to_string(),
        symbol: GRAHA_SYMBOLS[idx].to_string(),
        sidereal_lon: sid,
        rashi_idx: ri,
        rashi: RASHIS[ri].to_string(),
        nakshatra_idx: ni,
        nakshatra: NAKSHATRAS[ni].sanskrit.to_string(),
        is_retrograde: retro,
    }
}

pub fn get_all_graha_positions(jd: f64) -> Vec<GrahaPosition> {
    let ay = get_ayanamsa(jd);
    let t = julian_centuries(jd);
    let sun_l = normalize360(280.46646 + 36000.76983 * t);
    let sun_m = deg_to_rad(normalize360(357.52911 + 35999.05029 * t));
    let sun_lon = normalize360(sun_l + 1.9146 * sun_m.sin() + 0.02 * (2.0 * sun_m).sin());

    // सूर्य
    let surya = make_graha(0, sun_lon, ay, false);
    // चन्द्र
    let chandra = make_graha(1, get_moon_tropical(jd), ay, false);

    // मंगल
    let mars_l = normalize360(355.433 + 19140.2993 * t);
    let mars_m = deg_to_rad(normalize360(19.373 + 19139.8585 * t));
    let mars_lon = normalize360(mars_l + 10.691 * mars_m.sin() + 0.623 * (2.0 * mars_m).sin());
    let mars_elong = normalize360(mars_lon - sun_lon);
    let mangal = make_graha(2, mars_lon, ay, mars_elong > 120.0 && mars_elong < 240.0);

    // बुध
    let merc_m = deg_to_rad(normalize360(168.6562 + 4.0923344368 * (t * 36525.0)));
    let merc_lon = normalize360(sun_lon + 22.794 * merc_m.sin() + 0.660 * (2.0 * merc_m).sin());
    let budha = make_graha(3, merc_lon, ay, false);

    // गुरु
    let jup_l = normalize360(238.049 + 3034.9057 * t);
    let jup_m = deg_to_rad(normalize360(225.328 + 3034.6474 * t));
    let jup_lon = normalize360(jup_l + 5.555 * jup_m.sin() + 0.168 * (2.0 * jup_m).sin());
    let jup_elong = normalize360(jup_lon - sun_lon);
    let guru = make_graha(4, jup_lon, ay, jup_elong > 120.0 && jup_elong < 240.0);

    // शुक्र
    let ven_m = deg_to_rad(normalize360(50.4161 + 1.60213034 * (t * 36525.0)));
    let ven_lon = normalize360(sun_lon + 46.388 * ven_m.sin() + 0.290 * (2.0 * ven_m).sin());
    let shukra = make_graha(5, ven_lon, ay, false);

    // शनि
    let sat_l = normalize360(266.564 + 1222.1138 * t);
    let sat_m = deg_to_rad(normalize360(174.873 + 1221.5515 * t));
    let sat_lon = normalize360(sat_l + 6.399 * sat_m.sin() + 0.318 * (2.0 * sat_m).sin());
    let sat_elong = normalize360(sat_lon - sun_lon);
    let shani = make_graha(6, sat_lon, ay, sat_elong > 120.0 && sat_elong < 240.0);

    // राहु (सदैव वक्री)
    let rahu_lon = normalize360(125.0445 - 1934.1363 * t);
    let rahu = make_graha(7, rahu_lon, ay, true);

    // केतु (सदैव वक्री)
    let ketu_lon = normalize360(rahu_lon + 180.0);
    let ketu = make_graha(8, ketu_lon, ay, true);

    vec![surya, chandra, mangal, budha, guru, shukra, shani, rahu, ketu]
}
