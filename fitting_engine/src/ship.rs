use crate::faction::Faction;
use crate::ship_stats::ShipStats;
use crate::ship_type::ShipType;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship<'a> {
    pub name: Cow<'a, str>,
    pub ship_type: ShipType,
    pub faction: Faction,
    pub high_slots: u8,
    pub med_slots: u8,
    pub low_slots: u8,
    pub ship_stats: ShipStats,
}

impl Ship<'_> {
    pub fn new<T: Into<Cow<'static, str>>>(
        name: T,
        ship_type: ShipType,
        faction: Faction,
        high_slots: u8,
        med_slots: u8,
        low_slots: u8,
        ship_stats: ShipStats,
    ) -> Self {
        Self {
            name: name.into(),
            ship_type,
            faction,
            high_slots,
            med_slots,
            low_slots,
            ship_stats,
        }
    }
}
