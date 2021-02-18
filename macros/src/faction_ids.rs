use fitting_engine::faction::Faction;

pub fn faction_by_id(id: u64) -> Faction {
    match id {
        500001 => Faction::Caldari,
        500002 => Faction::Minmatar,
        500003 => Faction::Amarr,
        500004 => Faction::Gallente,
        500011 => Faction::AngelCartel,
        500010 => Faction::Guristas,
        500012 => Faction::BloodRaiders,
        500016 => Faction::SistersOfEve,
        500017 => Faction::SocietyofConsciousThought,
        500018 => Faction::MordusLegion,
        500019 => Faction::SanshasNation,
        500020 => Faction::Serpentis,
        500026 => Faction::Triglavians,
        500027 => Faction::EdenCom,
        500006 => Faction::Concord,
        500014 => Faction::ORE,
        _a => panic!("{} isnt a faction id", _a),
    }
}
