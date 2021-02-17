use crate::faction::Faction;
use crate::ship_stats::ShipStats;
use crate::ship_type::ShipType;

#[derive(Debug, Clone)]
pub struct Ship {
    name: String,
    ship_type: ShipType,
    faction: Faction,
    high_slots: u8,
    med_slots: u8,
    low_slots: u8,
    ship_stats: ShipStats,
}

impl Ship {
    pub fn new<T: ToString>(
        name: T,
        ship_type: ShipType,
        faction: Faction,
        high_slots: u8,
        med_slots: u8,
        low_slots: u8,
        ship_stats: ShipStats,
    ) -> Self {
        Self {
            name: name.to_string(),
            ship_type,
            faction,
            high_slots,
            med_slots,
            low_slots,
            ship_stats,
        }
    }
}
