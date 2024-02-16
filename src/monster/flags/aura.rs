use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterAura {
    Fire,
    Cold,
    Elec,
}

use MonsterAura::*;

const MONSTER_AURA_TABLE: [FlagTableRow<MonsterAura>; 3] = [
    FlagTableRow::new(Fire, "AURA_FIRE", "炎"),
    FlagTableRow::new(Cold, "AURA_COLD", "氷"),
    FlagTableRow::new(Elec, "AURA_ELEC", "スパーク"),
];

impl MonsterRaceFlag for MonsterAura {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_AURA_TABLE
    }
}
