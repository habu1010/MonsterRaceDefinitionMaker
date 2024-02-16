use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterWeakness {
    Fire,
    Cold,
    Lite,
    Rock,
}

use MonsterWeakness::*;

const MONSTER_WEAKNESS_TABLE: [FlagTableRow<MonsterWeakness>; 4] = [
    FlagTableRow::new(Fire, "HURT_FIRE", "炎"),
    FlagTableRow::new(Cold, "HURT_COLD", "冷気"),
    FlagTableRow::new(Lite, "HURT_LITE", "閃光"),
    FlagTableRow::new(Rock, "HURT_ROCK", "岩石溶解"),
];

impl MonsterRaceFlag for MonsterWeakness {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_WEAKNESS_TABLE
    }
}
