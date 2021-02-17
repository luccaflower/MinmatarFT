use crate::ship_type::stat_modification::{ModificationType, StatModification};
use num_traits::NumOps;

#[derive(Debug, Clone)]
pub struct ShipStats {
    shield_hp: usize,
    armor_hp: usize,
    hull_hp: usize,
    velocity: usize,
    agility: usize,
    mass: usize,
    power_grid: usize,
    cpu: usize,
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
            T: NumOps + Eq + Ord + Clone,
        {
            additions.sort();
            additions.into_iter().fold(base_val, |acc, x| x.apply(acc))
        }
        let mut r = self.clone();
        let (
            shield_hp,
            armor_hp,
            hull_hp,
            velocity,
            agility,
            mass,
            power_grid,
            cpu,
        ) = stat_mods.into_iter().fold(
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
