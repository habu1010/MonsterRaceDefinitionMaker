use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterPopulation {
    Nazgul,
    OnlyOne,
}

use MonsterPopulation::*;

const MONSTER_POPULATION_TABLE: [FlagTableRow<MonsterPopulation>; 2] = [
    FlagTableRow::new(Nazgul, "NAZGUL", "ナズグル"),
    FlagTableRow::new(OnlyOne, "ONLY_ONE", "一体のみ"),
];

impl MonsterRaceFlag for MonsterPopulation {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_POPULATION_TABLE
    }
}
