use crate::model::dogma_attribute::DogmaAttribute;
use crate::model::group_id::GroupId;
use fitting_engine::stats::ModificationType;
use std::collections::HashMap;

pub fn extract_resistance(
    v: &HashMap<u64, (f64, &DogmaAttribute)>,
    g: &GroupId,
    hull_id: u64,
    armor_id: u64,
    shield_id: u64,
    additive_id: u64,
) -> (
    (
        ModificationType<f64>,
        ModificationType<f64>,
        ModificationType<f64>,
    ),
    (
        ModificationType<f64>,
        ModificationType<f64>,
        ModificationType<f64>,
    ),
    bool,
) {
    let (hull, armor, shield, active) =
        extract_resistance_raw(v, g, hull_id, armor_id, shield_id, additive_id);
    if let Some((activated_damage_resistance, _)) = v.get(&2746) {
        let adr = ModificationType::Additive(*activated_damage_resistance);
        (
            (hull, armor, shield),
            (adr.clone(), adr.clone(), adr),
            active,
        )
    } else {
        if active {
            (
                (
                    ModificationType::default(),
                    ModificationType::default(),
                    ModificationType::default(),
                ),
                (hull, armor, shield),
                active,
            )
        } else {
            (
                (hull, armor, shield),
                (
                    ModificationType::default(),
                    ModificationType::default(),
                    ModificationType::default(),
                ),
                active,
            )
        }
    }
}

pub fn extract_resistance_raw(
    v: &HashMap<u64, (f64, &DogmaAttribute)>,
    g: &GroupId,
    hull_id: u64,
    armor_id: u64,
    shield_id: u64,
    additive_id: u64,
) -> (
    ModificationType<f64>,
    ModificationType<f64>,
    ModificationType<f64>,
    bool,
) {
    let active = v.get(&73).is_some();
    match (v.get(&hull_id), v.get(&armor_id), v.get(&shield_id)) {
        (Some((armor, _)), Some((shield, _)), Some((hull, _))) => (
            ModificationType::Multiplicative(*armor),
            ModificationType::Multiplicative(*shield),
            ModificationType::Multiplicative(*hull),
            active,
        ),
        (Some((armor, _)), Some((shield, _)), None) => (
            ModificationType::Multiplicative(*armor),
            ModificationType::Multiplicative(*shield),
            ModificationType::default(),
            active,
        ),
        (Some((armor, _)), None, Some((hull, _))) => (
            ModificationType::Multiplicative(*armor),
            ModificationType::default(),
            ModificationType::Multiplicative(*hull),
            active,
        ),
        (None, Some((shield, _)), Some((hull, _))) => (
            ModificationType::default(),
            ModificationType::Multiplicative(*shield),
            ModificationType::Multiplicative(*hull),
            active,
        ),
        (Some((armor, _)), None, None) => (
            ModificationType::Multiplicative(*armor),
            ModificationType::default(),
            ModificationType::default(),
            active,
        ),
        (None, Some((shield, _)), None) => (
            ModificationType::default(),
            ModificationType::Multiplicative(*shield),
            ModificationType::default(),
            active,
        ),
        (None, None, Some((hull, _))) => (
            ModificationType::default(),
            ModificationType::default(),
            ModificationType::Multiplicative(*hull),
            active,
        ),
        (None, None, None) => match v.get(&additive_id) {
            None => (
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
                active,
            ),
            Some((v, _)) => match g.name.en.as_ref().unwrap().as_ref() {
                "Shield Hardener" => (
                    ModificationType::default(),
                    ModificationType::Additive(*v),
                    ModificationType::default(),
                    active,
                ),
                "Armor Hardener" => (
                    ModificationType::Additive(*v),
                    ModificationType::default(),
                    ModificationType::default(),
                    active,
                ),
                _ => (
                    ModificationType::default(),
                    ModificationType::default(),
                    ModificationType::default(),
                    active,
                ),
            },
        },
    }
}
