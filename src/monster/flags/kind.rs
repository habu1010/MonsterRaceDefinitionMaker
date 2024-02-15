use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterKind {
    Unique,
    Human,
    Quantum,
    Orc,
    Troll,
    Giant,
    Dragon,
    Demon,
    Undead,
    Evil,
    Animal,
    Amberite,
    Good,
    Nonliving,
    Angel,
}

use MonsterKind::*;

pub const MONSTER_KIND_TABLES: [FlagTable<MonsterKind>; 15] = [
    FlagTable::new(Unique, "UNIQUE", "ユニーク"),
    FlagTable::new(Human, "HUMAN", "人間"),
    FlagTable::new(Quantum, "QUANTUM", "量子生物"),
    FlagTable::new(Orc, "ORC", "オーク"),
    FlagTable::new(Troll, "TROLL", "トロル"),
    FlagTable::new(Giant, "GIANT", "巨人"),
    FlagTable::new(Dragon, "DRAGON", "ドラゴン"),
    FlagTable::new(Demon, "DEMON", "悪魔"),
    FlagTable::new(Undead, "UNDEAD", "アンデッド"),
    FlagTable::new(Evil, "EVIL", "邪悪"),
    FlagTable::new(Animal, "ANIMAL", "動物"),
    FlagTable::new(Amberite, "AMBERITE", "アンバーの王族"),
    FlagTable::new(Good, "GOOD", "善良"),
    FlagTable::new(Nonliving, "NONLIVING", "非生物"),
    FlagTable::new(Angel, "ANGEL", "天使"),
];

impl MonsterRaceFlag for MonsterKind {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_KIND_TABLES
    }
}
