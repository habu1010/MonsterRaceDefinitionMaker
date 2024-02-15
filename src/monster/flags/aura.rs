use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterAura {
    Fire,
    Cold,
    Elec,
}

use MonsterAura::*;

pub const MONSTER_AURA_TABLES: [FlagTable<MonsterAura>; 3] = [
    FlagTable::new(Fire, "AURA_FIRE", "炎"),
    FlagTable::new(Cold, "AURA_COLD", "氷"),
    FlagTable::new(Elec, "AURA_ELEC", "スパーク"),
];

impl MonsterRaceFlag for MonsterAura {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_AURA_TABLES
    }
}
