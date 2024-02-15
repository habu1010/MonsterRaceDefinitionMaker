use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterPopulation {
    Nazgul,
    OnlyOne,
}

use MonsterPopulation::*;

pub const MONSTER_POPULATION_TABLES: [FlagTable<MonsterPopulation>; 2] = [
    FlagTable::new(Nazgul, "NAZGUL", "ナズグル"),
    FlagTable::new(OnlyOne, "ONLY_ONE", "一体のみ"),
];

impl MonsterRaceFlag for MonsterPopulation {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_POPULATION_TABLES
    }
}
