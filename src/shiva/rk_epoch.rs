/// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
///  RK कालगणना यन्त्र — राधा-कृष्ण दिव्य संवत्
///  T₀ = भाद्रपद कृष्ण अष्टमी, ३२२७ ईसापूर्व
/// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
use crate::shiva::vedic_data::TROPICAL_YEAR;
use serde::Serialize;

/// ग्रेगोरियन → ज्यूलियन दिवसांक
pub fn gregorian_to_jd(year: i32, month: u32, day: u32, hour: f64) -> f64 {
    let day_frac = hour / 24.0;
    let (mut y, mut m) = (year as i64, month as i64);
    if m <= 2 { y -= 1; m += 12; }
    let b: i64 = if year > 1582 || (year == 1582 && month > 10) || (year == 1582 && month == 10 && day >= 15) {
        let a = y / 100;
        2 - a + a / 4
    } else { 0 };
    (365.25 * (y + 4716) as f64).floor()
        + (30.6001 * (m + 1) as f64).floor()
        + day as f64 + day_frac + b as f64 - 1524.5
}

/// T₀ ज्यूलियन दिवसांक
pub fn t0_jd() -> f64 { gregorian_to_jd(-3226, 8, 11, 6.0) }

/// J2000.0 से ज्यूलियन शताब्दी
pub fn julian_centuries(jd: f64) -> f64 { (jd - 2451545.0) / 36525.0 }

#[derive(Debug, Clone, Serialize)]
pub struct RKTimestamp {
    pub rk_year: i64,
    pub elapsed_days: f64,
    pub day_of_year: f64,
    pub jd: f64,
}

pub fn get_rk_timestamp(jd: f64) -> RKTimestamp {
    let elapsed = jd - t0_jd();
    let rk_year = (elapsed / TROPICAL_YEAR).floor() as i64 + 1;
    let day_of_year = elapsed - ((rk_year - 1) as f64 * TROPICAL_YEAR);
    RKTimestamp { rk_year, elapsed_days: elapsed, day_of_year, jd }
}

#[derive(Debug, Clone, Serialize)]
pub struct RKElapsed {
    pub years: i64,
    pub months: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

pub fn format_rk_elapsed(elapsed_days: f64) -> RKElapsed {
    let years = (elapsed_days / TROPICAL_YEAR).floor() as i64 + 1;
    let rem = elapsed_days - ((years - 1) as f64 * TROPICAL_YEAR);
    let months = (rem / crate::shiva::vedic_data::SYNODIC_MONTH).floor() as i64;
    let days = (rem - months as f64 * crate::shiva::vedic_data::SYNODIC_MONTH).floor() as i64;
    let frac = rem - rem.floor();
    let hours = (frac * 24.0).floor() as i64;
    let minutes = ((frac * 24.0 - hours as f64) * 60.0).floor() as i64;
    let seconds = (((frac * 24.0 - hours as f64) * 60.0 - minutes as f64) * 60.0).floor() as i64;
    RKElapsed { years, months, days, hours, minutes, seconds }
}

/// System Time to Julian Day
pub fn system_time_to_jd() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let days_since_epoch = since_the_epoch.as_secs() as f64 / 86400.0;
    days_since_epoch + 2440587.5
}
