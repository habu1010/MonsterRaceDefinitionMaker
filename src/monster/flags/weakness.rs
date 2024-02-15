use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterWeakness {
    Fire,
    Cold,
    Lite,
    Rock,
}

use MonsterWeakness::*;

pub const MONSTER_WEAKNESS_TABLES: [FlagTable<MonsterWeakness>; 4] = [
    FlagTable::new(Fire, "HURT_FIRE", "炎"),
    FlagTable::new(Cold, "HURT_COLD", "冷気"),
    FlagTable::new(Lite, "HURT_LITE", "閃光"),
    FlagTable::new(Rock, "HURT_ROCK", "岩石溶解"),
];

impl MonsterRaceFlag for MonsterWeakness {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_WEAKNESS_TABLES
    }
}
