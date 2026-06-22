use std::time::{SystemTime, UNIX_EPOCH};

/// Coordinate in the 3D Cosmic Shard Grid
/// Total shards = 14 (Shiva) × 27 (Nakshatra) × 12 (Rashi) = 4,536 shards
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ShardCoordinate {
    pub shiva_dimension: u8, // 0-13
    pub nakshatra_sector: u8, // 0-26
    pub rashi_ring: u8, // 0-11
}

impl ShardCoordinate {
    pub fn to_string_key(&self) -> String {
        format!("{}-{}-{}", self.shiva_dimension, self.nakshatra_sector, self.rashi_ring)
    }
}

pub struct ShardRouter;

impl ShardRouter {
    /// Calculate the 3D coordinate for a given key and timestamp
    pub fn calculate_coordinate(key: &str) -> ShardCoordinate {
        let mut hash: u64 = 5381;
        for b in key.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64);
        }

        // Add cosmic time variance
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        // Shift hash slightly based on current hour to simulate cosmic rotation
        let cosmic_hash = hash.wrapping_add(now / 3600);

        ShardCoordinate {
            shiva_dimension: (cosmic_hash % 14) as u8,
            nakshatra_sector: ((cosmic_hash / 14) % 27) as u8,
            rashi_ring: ((cosmic_hash / (14 * 27)) % 12) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_coordinate_to_string_key() {
        let coord = ShardCoordinate {
            shiva_dimension: 13,
            nakshatra_sector: 26,
            rashi_ring: 11,
        };
        assert_eq!(coord.to_string_key(), "13-26-11");
    }

    #[test]
    fn test_calculate_coordinate_bounds() {
        // Test many keys to ensure they never exceed grid bounds
        for i in 0..1000 {
            let key = format!("test_key_bound_{}", i);
            let coord = ShardRouter::calculate_coordinate(&key);
            assert!(coord.shiva_dimension < 14);
            assert!(coord.nakshatra_sector < 27);
            assert!(coord.rashi_ring < 12);
        }
    }

    #[test]
    fn test_coordinate_distribution() {
        // Simple test to ensure not all keys map to the same shard
        let mut shards = HashSet::new();
        for i in 0..100 {
            let key = format!("distribution_test_{}", i);
            let coord = ShardRouter::calculate_coordinate(&key);
            shards.insert(coord.to_string_key());
        }
        // At least 90 out of 100 should be unique due to hash avalanche
        assert!(shards.len() > 90);
    }

    #[test]
    fn test_shard_coordinate_equality() {
        let c1 = ShardCoordinate { shiva_dimension: 5, nakshatra_sector: 10, rashi_ring: 2 };
        let c2 = ShardCoordinate { shiva_dimension: 5, nakshatra_sector: 10, rashi_ring: 2 };
        let c3 = ShardCoordinate { shiva_dimension: 5, nakshatra_sector: 10, rashi_ring: 3 };

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }
}
