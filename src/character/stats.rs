// src/character/stats.rs
// Character statistics and calculations

use serde::{Deserialize, Serialize};

// ==========================================
// STAT POOLS
// ==========================================

/// Represents the three stat pools: Might, Speed, Intellect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pools {
    pub might: i32,
    pub speed: i32,
    pub intellect: i32,
}

impl Pools {
    /// Create new pools with given values
    pub fn new(might: i32, speed: i32, intellect: i32) -> Self {
        Self {
            might,
            speed,
            intellect,
        }
    }

    /// Create pools with all zeros
    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Add another set of pools to this one
    pub fn add(&mut self, other: Pools) {
        self.might += other.might;
        self.speed += other.speed;
        self.intellect += other.intellect;
    }

    /// Get total points across all pools
    pub fn total(&self) -> i32 {
        self.might + self.speed + self.intellect
    }

    /// Check if all pools are valid (>= 0)
    pub fn is_valid(&self) -> bool {
        self.might >= 0 && self.speed >= 0 && self.intellect >= 0
    }

    /// Get pool value by name
    pub fn get_pool(&self, pool_name: &str) -> Option<i32> {
        match pool_name.to_lowercase().as_str() {
            "might" => Some(self.might),
            "speed" => Some(self.speed),
            "intellect" => Some(self.intellect),
            _ => None,
        }
    }

    /// Set pool value by name
    pub fn set_pool(&mut self, pool_name: &str, value: i32) -> bool {
        match pool_name.to_lowercase().as_str() {
            "might" => {
                self.might = value;
                true
            }
            "speed" => {
                self.speed = value;
                true
            }
            "intellect" => {
                self.intellect = value;
                true
            }
            _ => false,
        }
    }
}

impl std::fmt::Display for Pools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Might: {}, Speed: {}, Intellect: {}",
            self.might, self.speed, self.intellect
        )
    }
}

// ==========================================
// EDGE VALUES
// ==========================================

/// Represents Edge values for each stat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    pub might: u32,
    pub speed: u32,
    pub intellect: u32,
}

impl Edge {
    /// Create new edge values
    pub fn new(might: u32, speed: u32, intellect: u32) -> Self {
        Self {
            might,
            speed,
            intellect,
        }
    }

    /// Create edge with all zeros
    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Get edge value by name
    pub fn get_edge(&self, pool_name: &str) -> Option<u32> {
        match pool_name.to_lowercase().as_str() {
            "might" => Some(self.might),
            "speed" => Some(self.speed),
            "intellect" => Some(self.intellect),
            _ => None,
        }
    }

    /// Calculate effective cost after applying edge
    pub fn apply_to_cost(&self, pool_name: &str, cost: u32) -> u32 {
        let edge = self.get_edge(pool_name).unwrap_or(0);
        cost.saturating_sub(edge)
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Might: {}, Speed: {}, Intellect: {}",
            self.might, self.speed, self.intellect
        )
    }
}

// ==========================================
// EFFORT
// ==========================================

/// Represents effort capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effort {
    /// Maximum effort levels available (based on tier)
    pub max_effort: u32,
}

impl Effort {
    /// Create new effort
    pub fn new(max_effort: u32) -> Self {
        Self { max_effort }
    }

    /// Calculate cost to apply effort levels
    /// First level costs 3, each additional costs 2
    pub fn calculate_cost(&self, levels: u32) -> u32 {
        if levels == 0 {
            0
        } else {
            3 + (levels.saturating_sub(1) * 2)
        }
    }

    /// Calculate damage bonus from effort levels
    /// Each level adds +3 damage
    pub fn damage_bonus(&self, levels: u32) -> u32 {
        levels * 3
    }

    /// Check if effort levels are valid
    pub fn is_valid(&self, levels: u32) -> bool {
        levels <= self.max_effort
    }
}

// ==========================================
// DAMAGE TRACK
// ==========================================

/// Represents the damage track state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageTrack {
    Hale,
    Impaired,
    Debilitated,
    Dead,
}

impl DamageTrack {
    /// Get the next worse state
    pub fn worsen(&self) -> Self {
        match self {
            DamageTrack::Hale => DamageTrack::Impaired,
            DamageTrack::Impaired => DamageTrack::Debilitated,
            DamageTrack::Debilitated => DamageTrack::Dead,
            DamageTrack::Dead => DamageTrack::Dead,
        }
    }

    /// Get the next better state
    pub fn improve(&self) -> Self {
        match self {
            DamageTrack::Dead => DamageTrack::Dead,
            DamageTrack::Debilitated => DamageTrack::Impaired,
            DamageTrack::Impaired => DamageTrack::Hale,
            DamageTrack::Hale => DamageTrack::Hale,
        }
    }

    /// Get task difficulty modifier for current state
    pub fn difficulty_modifier(&self) -> i32 {
        match self {
            DamageTrack::Hale => 0,
            DamageTrack::Impaired => 1,    // Tasks hindered by 1 step
            DamageTrack::Debilitated => 1, // Tasks hindered by 1 step
            DamageTrack::Dead => 999,      // Cannot act
        }
    }

    /// Get movement modifier for current state
    pub fn movement_modifier(&self) -> f32 {
        match self {
            DamageTrack::Hale => 1.0,
            DamageTrack::Impaired => 1.0,
            DamageTrack::Debilitated => 0.0, // Cannot move more than immediate distance
            DamageTrack::Dead => 0.0,
        }
    }

    /// Check if character can act
    pub fn can_act(&self) -> bool {
        !matches!(self, DamageTrack::Dead)
    }

    /// Get description of current state
    pub fn description(&self) -> &str {
        match self {
            DamageTrack::Hale => "Healthy and unharmed",
            DamageTrack::Impaired => "Injured - all tasks hindered by one step",
            DamageTrack::Debilitated => {
                "Severely wounded - all tasks hindered, cannot move more than immediate distance"
            }
            DamageTrack::Dead => "Dead",
        }
    }
}

impl std::fmt::Display for DamageTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageTrack::Hale => write!(f, "Hale"),
            DamageTrack::Impaired => write!(f, "Impaired"),
            DamageTrack::Debilitated => write!(f, "Debilitated"),
            DamageTrack::Dead => write!(f, "Dead"),
        }
    }
}

// ==========================================
// STAT CALCULATIONS
// ==========================================

/// Calculate final stat pools from various sources
pub fn calculate_pools(
    base_pools: Pools,
    descriptor_mods: Pools,
    species_mods: Pools,
    bonus_points: Pools,
) -> Pools {
    let mut final_pools = base_pools;
    final_pools.add(descriptor_mods);
    final_pools.add(species_mods);
    final_pools.add(bonus_points);
    final_pools
}

/// Determine damage track state based on pool values
pub fn determine_damage_track(pools: Pools) -> DamageTrack {
    // Count how many pools are at 0
    let zeros = [pools.might, pools.speed, pools.intellect]
        .iter()
        .filter(|&&p| p == 0)
        .count();

    match zeros {
        0 => DamageTrack::Hale,
        1 => DamageTrack::Impaired,
        2 => DamageTrack::Debilitated,
        _ => DamageTrack::Dead,
    }
}

/// Calculate effective armor value
pub fn calculate_armor(base_armor: u32, additional_armor: &[u32]) -> u32 {
    base_armor + additional_armor.iter().sum::<u32>()
}

// ==========================================
// RECOVERY ROLLS
// ==========================================

/// Represents a recovery roll
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRoll {
    /// Die roll result (1d6 + tier)
    pub roll: u32,
    /// Character tier
    pub tier: u32,
}

impl RecoveryRoll {
    /// Create new recovery roll
    pub fn new(roll: u32, tier: u32) -> Self {
        Self { roll, tier }
    }

    /// Calculate total recovery points
    pub fn total(&self) -> u32 {
        self.roll + self.tier
    }

    /// Roll recovery (simulated as average for character creation)
    pub fn simulate(tier: u32) -> Self {
        // Use average roll (3.5 rounded to 4) for character creation
        Self::new(4, tier)
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_add() {
        let mut pools = Pools::new(10, 10, 8);
        pools.add(Pools::new(2, 0, 4));
        assert_eq!(pools.might, 12);
        assert_eq!(pools.speed, 10);
        assert_eq!(pools.intellect, 12);
    }

    #[test]
    fn test_pools_total() {
        let pools = Pools::new(10, 10, 8);
        assert_eq!(pools.total(), 28);
    }

    #[test]
    fn test_edge_apply_to_cost() {
        let edge = Edge::new(1, 2, 0);
        assert_eq!(edge.apply_to_cost("might", 3), 2);
        assert_eq!(edge.apply_to_cost("speed", 3), 1);
        assert_eq!(edge.apply_to_cost("intellect", 3), 3);
        assert_eq!(edge.apply_to_cost("speed", 1), 0); // Edge reduces to 0
    }

    #[test]
    fn test_effort_cost() {
        let effort = Effort::new(3);
        assert_eq!(effort.calculate_cost(0), 0);
        assert_eq!(effort.calculate_cost(1), 3);
        assert_eq!(effort.calculate_cost(2), 5);
        assert_eq!(effort.calculate_cost(3), 7);
    }

    #[test]
    fn test_effort_damage() {
        let effort = Effort::new(3);
        assert_eq!(effort.damage_bonus(0), 0);
        assert_eq!(effort.damage_bonus(1), 3);
        assert_eq!(effort.damage_bonus(2), 6);
        assert_eq!(effort.damage_bonus(3), 9);
    }

    #[test]
    fn test_damage_track_progression() {
        let mut state = DamageTrack::Hale;
        assert_eq!(state.worsen(), DamageTrack::Impaired);

        state = state.worsen();
        assert_eq!(state, DamageTrack::Impaired);
        assert_eq!(state.worsen(), DamageTrack::Debilitated);

        state = state.worsen();
        assert_eq!(state, DamageTrack::Debilitated);
        assert_eq!(state.worsen(), DamageTrack::Dead);
    }

    #[test]
    fn test_determine_damage_track() {
        assert_eq!(
            determine_damage_track(Pools::new(10, 10, 8)),
            DamageTrack::Hale
        );
        assert_eq!(
            determine_damage_track(Pools::new(0, 10, 8)),
            DamageTrack::Impaired
        );
        assert_eq!(
            determine_damage_track(Pools::new(0, 0, 8)),
            DamageTrack::Debilitated
        );
        assert_eq!(
            determine_damage_track(Pools::new(0, 0, 0)),
            DamageTrack::Dead
        );
    }

    #[test]
    fn test_calculate_pools() {
        let base = Pools::new(10, 10, 8);
        let descriptor = Pools::new(2, 0, 0);
        let species = Pools::new(0, 0, 2);
        let bonus = Pools::new(4, 2, 0);

        let final_pools = calculate_pools(base, descriptor, species, bonus);
        assert_eq!(final_pools.might, 16);
        assert_eq!(final_pools.speed, 12);
        assert_eq!(final_pools.intellect, 10);
    }

    #[test]
    fn test_calculate_armor() {
        assert_eq!(calculate_armor(0, &[]), 0);
        assert_eq!(calculate_armor(1, &[]), 1);
        assert_eq!(calculate_armor(1, &[2, 1]), 4);
        assert_eq!(calculate_armor(0, &[1, 1, 1]), 3);
    }

    #[test]
    fn test_recovery_roll() {
        let roll = RecoveryRoll::new(4, 1);
        assert_eq!(roll.total(), 5);

        let simulated = RecoveryRoll::simulate(2);
        assert_eq!(simulated.total(), 6); // 4 (avg roll) + 2 (tier)
    }
}
