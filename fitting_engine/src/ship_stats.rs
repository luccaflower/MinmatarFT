use crate::ship_type::stat_modification::{ModificationType, StatModification};
use num_traits::{NumOps, Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStats {
    pub shield_hp: usize,
    pub armor_hp: usize,
    pub hull_hp: usize,
    pub velocity: usize,
    pub agility: usize,
    pub mass: usize,
    pub power_grid: usize,
    pub cpu: usize,
}

impl ShipStats {
    pub fn new(
        shield_hp: usize,
        armor_hp: usize,
        hull_hp: usize,
        velocity: usize,
        agility: usize,
        mass: usize,
        power_grid: usize,
        cpu: usize,
    ) -> Self {
        Self {
            shield_hp,
            armor_hp,
            hull_hp,
            velocity,
            agility,
            mass,
            power_grid,
            cpu,
        }
    }

    pub fn apply(&self, stat_mods: Vec<&StatModification>) -> Self {
        fn calculate<T>(base_val: T, mut additions: Vec<&ModificationType<T>>) -> T
        where
            T: NumOps + Eq + Ord + Clone + Zero,
        {
            additions.sort();
            additions.into_iter().fold(base_val, |acc, x| x.apply(acc))
        }
        let mut r = self.clone();
        let (shield_hp, armor_hp, hull_hp, velocity, agility, mass, power_grid, cpu) =
            stat_mods.into_iter().fold(
                (
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ),
                |(
                    mut shield_hp_vec,
                    mut armor_hp_vec,
                    mut hull_hp_vec,
                    mut velocity_vec,
                    mut agility_vec,
                    mut mass_vec,
                    mut power_grid_vec,
                    mut cpu_vec,
                ),
                 x| {
                    let StatModification {
                        shield_hp,
                        armor_hp,
                        hull_hp,
                        velocity,
                        agility,
                        mass,
                        power_grid,
                        cpu,
                    } = x;
                    shield_hp_vec.push(shield_hp);
                    armor_hp_vec.push(armor_hp);
                    hull_hp_vec.push(hull_hp);
                    velocity_vec.push(velocity);
                    agility_vec.push(agility);
                    mass_vec.push(mass);
                    power_grid_vec.push(power_grid);
                    cpu_vec.push(cpu);
                    (
                        shield_hp_vec,
                        armor_hp_vec,
                        hull_hp_vec,
                        velocity_vec,
                        agility_vec,
                        mass_vec,
                        power_grid_vec,
                        cpu_vec,
                    )
                },
            );
        r.shield_hp = calculate(r.shield_hp, shield_hp);
        r.armor_hp = calculate(r.armor_hp, armor_hp);
        r.hull_hp = calculate(r.hull_hp, hull_hp);
        r.velocity = calculate(r.velocity, velocity);
        r.agility = calculate(r.agility, agility);
        r.mass = calculate(r.mass, mass);
        r.power_grid = calculate(r.power_grid, power_grid);
        r.cpu = calculate(r.cpu, cpu);
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::ship_stats::ShipStats;
    use crate::ship_type::stat_modification::ModificationType::Additive;
    use crate::ship_type::stat_modification::StatModification;

    fn stats() -> ShipStats {
        ShipStats::new(1000, 1000, 1000, 100, 100, 10000, 100, 100)
    }

    #[test]
    fn applies_single_additive_mod() {
        let mut stat_mods = StatModification::default();
        stat_mods.hull_hp = Additive(500);
        assert_eq!(1500, stats().apply(vec![&stat_mods]).hull_hp)
    }

    #[test]
    fn applies_multiple_additive_stats_from_single_mod() {
        let mut stat_mods = StatModification::default();
        stat_mods.hull_hp = Additive(500);
        stat_mods.mass = Additive(1000);
        let modified_stats = stats().apply(vec![&stat_mods]);
        assert_eq!(1500, modified_stats.hull_hp);
        assert_eq!(11000, modified_stats.mass);
    }

    #[test]
    fn applies_additive_stats_from_multiple_mods() {
        let mut stat_mods1 = StatModification::default();
        let mut stat_mods2 = StatModification::default();
        stat_mods1.velocity = Additive(20);
        stat_mods2.cpu = Additive(50);
        let modified_stats = stats().apply(vec![&stat_mods1, &stat_mods2]);
        assert_eq!(120, modified_stats.velocity);
        assert_eq!(150, modified_stats.cpu);
    }
}
