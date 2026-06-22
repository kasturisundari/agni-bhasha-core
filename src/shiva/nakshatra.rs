/// # Lunar Mansions — The 27 Nakshatras
///
/// Represents celestial frequencies that control consensus and block mining in Kasturichain.
/// Instead of wasting energy on random calculations, consensus relies on alignment with cosmic time.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nakshatra {
    Ashvini = 1,
    Bharani = 2,
    Krittika = 3,
    Rohini = 4,
    Mrigashirsha = 5,
    Ardra = 6,
    Punarvasu = 7,
    Pushya = 8,
    Ashlesha = 9,
    Magha = 10,
    PurvaPhalguni = 11,
    UttaraPhalguni = 12,
    Hasta = 13,
    Chitra = 14,
    Swati = 15,
    Vishakha = 16,
    Anuradha = 17,
    Jyeshtha = 18,
    Mula = 19,
    PurvaAshadha = 20,
    UttaraAshadha = 21,
    Shravana = 22,
    Dhanishta = 23,
    Shatabhisha = 24,
    PurvaBhadrapada = 25,
    UttaraBhadrapada = 26,
    Revati = 27,
}

impl Nakshatra {
    /// Get the current Nakshatra based on unix timestamp
    pub fn current_from_time(timestamp_ms: u64) -> Self {
        // Small scale cycle: each Nakshatra rules for 10 seconds
        // 27 Nakshatras * 10 = 270 seconds cycle
        let seconds = timestamp_ms / 1000;
        let index = (seconds / 10) % 27;
        Self::from_index((index + 1) as usize).unwrap_or(Self::Ashvini)
    }

    /// Convert index to Nakshatra enum
    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            1 => Some(Self::Ashvini),
            2 => Some(Self::Bharani),
            3 => Some(Self::Krittika),
            4 => Some(Self::Rohini),
            5 => Some(Self::Mrigashirsha),
            6 => Some(Self::Ardra),
            7 => Some(Self::Punarvasu),
            8 => Some(Self::Pushya),
            9 => Some(Self::Ashlesha),
            10 => Some(Self::Magha),
            11 => Some(Self::PurvaPhalguni),
            12 => Some(Self::UttaraPhalguni),
            13 => Some(Self::Hasta),
            14 => Some(Self::Chitra),
            15 => Some(Self::Swati),
            16 => Some(Self::Vishakha),
            17 => Some(Self::Anuradha),
            18 => Some(Self::Jyeshtha),
            19 => Some(Self::Mula),
            20 => Some(Self::PurvaAshadha),
            21 => Some(Self::UttaraAshadha),
            22 => Some(Self::Shravana),
            23 => Some(Self::Dhanishta),
            24 => Some(Self::Shatabhisha),
            25 => Some(Self::PurvaBhadrapada),
            26 => Some(Self::UttaraBhadrapada),
            27 => Some(Self::Revati),
            _ => None,
        }
    }

    /// Cosmic Harmonic Frequency associated with the Nakshatra
    /// Used in Resonance Mining
    pub fn frequency(&self) -> f64 {
        // Schumann Resonance base ~ 7.83 Hz
        let base_hz = 7.83;
        base_hz * (*self as u32 as f64)
    }

    /// Sanskrit Devanagari Name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Ashvini => "अश्विनी",
            Self::Bharani => "भरणी",
            Self::Krittika => "कृत्तिका",
            Self::Rohini => "रोहिणी",
            Self::Mrigashirsha => "मृगशीर्षा",
            Self::Ardra => "आर्द्रा",
            Self::Punarvasu => "पुनर्वसु",
            Self::Pushya => "पुष्य",
            Self::Ashlesha => "आश्लेषा",
            Self::Magha => "मघा",
            Self::PurvaPhalguni => "पूर्वफाल्गुनी",
            Self::UttaraPhalguni => "उत्तरफाल्गुनी",
            Self::Hasta => "हस्त",
            Self::Chitra => "चित्रा",
            Self::Swati => "स्वाती",
            Self::Vishakha => "विशाखा",
            Self::Anuradha => "अनुराधा",
            Self::Jyeshtha => "ज्येष्ठा",
            Self::Mula => "मूल",
            Self::PurvaAshadha => "पूर्वाषाढा",
            Self::UttaraAshadha => "उत्तराषाढा",
            Self::Shravana => "श्रवण",
            Self::Dhanishta => "धनिष्ठा",
            Self::Shatabhisha => "शतभिषा",
            Self::PurvaBhadrapada => "पूर्वभाद्रपदा",
            Self::UttaraBhadrapada => "उत्तरभाद्रपदा",
            Self::Revati => "रेवती",
        }
    }
}

/// Get the current Nakshatra based on the current system time
pub fn get_current_nakshatra() -> Nakshatra {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    Nakshatra::current_from_time(now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nakshatra_from_index() {
        assert_eq!(Nakshatra::from_index(1), Some(Nakshatra::Ashvini));
        assert_eq!(Nakshatra::from_index(14), Some(Nakshatra::Chitra));
        assert_eq!(Nakshatra::from_index(27), Some(Nakshatra::Revati));
        assert_eq!(Nakshatra::from_index(0), None);
        assert_eq!(Nakshatra::from_index(28), None);
    }

    #[test]
    fn test_nakshatra_sanskrit_names() {
        assert_eq!(Nakshatra::Ashvini.sanskrit_name(), "अश्विनी");
        assert_eq!(Nakshatra::Rohini.sanskrit_name(), "रोहिणी");
        assert_eq!(Nakshatra::Revati.sanskrit_name(), "रेवती");
    }

    #[test]
    fn test_nakshatra_frequency_calculation() {
        let f1 = Nakshatra::Ashvini.frequency();
        assert!((f1 - 7.83).abs() < f64::EPSILON);

        let f10 = Nakshatra::Magha.frequency();
        assert!((f10 - 78.3).abs() < f64::EPSILON);

        let f27 = Nakshatra::Revati.frequency();
        assert!((f27 - (7.83 * 27.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_nakshatra_current_from_time() {
        // time = 0 -> 0 seconds -> index = 0 -> +1 = 1 -> Ashvini
        assert_eq!(Nakshatra::current_from_time(0), Nakshatra::Ashvini);
        
        // time = 10000 -> 10 seconds -> index = 1 -> +1 = 2 -> Bharani
        assert_eq!(Nakshatra::current_from_time(10000), Nakshatra::Bharani);

        // time = 260000 -> 260 seconds -> index = 26 -> +1 = 27 -> Revati
        assert_eq!(Nakshatra::current_from_time(260000), Nakshatra::Revati);

        // Wrap around: 270 seconds -> index 0
        assert_eq!(Nakshatra::current_from_time(270000), Nakshatra::Ashvini);
    }

    #[test]
    fn test_nakshatra_exhaustiveness() {
        // Ensure all 27 nakshatras can be generated from time
        let mut seen = std::collections::HashSet::new();
        for sec in 0..270 {
            seen.insert(Nakshatra::current_from_time(sec * 1000));
        }
        assert_eq!(seen.len(), 27);
    }

    #[test]
    fn test_frequency_is_positive() {
        for i in 1..=27 {
            let nak = Nakshatra::from_index(i).unwrap();
            assert!(nak.frequency() > 0.0);
        }
    }

    #[test]
    fn test_nakshatra_enum_values() {
        assert_eq!(Nakshatra::Ashvini as u32, 1);
        assert_eq!(Nakshatra::Krittika as u32, 3);
        assert_eq!(Nakshatra::Chitra as u32, 14);
        assert_eq!(Nakshatra::Revati as u32, 27);
    }
}
