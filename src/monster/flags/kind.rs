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

const MONSTER_KIND_TABLE: [FlagTableRow<MonsterKind>; 15] = [
    FlagTableRow::new(Unique, "UNIQUE", "ユニーク"),
    FlagTableRow::new(Human, "HUMAN", "人間"),
    FlagTableRow::new(Quantum, "QUANTUM", "量子生物"),
    FlagTableRow::new(Orc, "ORC", "オーク"),
    FlagTableRow::new(Troll, "TROLL", "トロル"),
    FlagTableRow::new(Giant, "GIANT", "巨人"),
    FlagTableRow::new(Dragon, "DRAGON", "ドラゴン"),
    FlagTableRow::new(Demon, "DEMON", "悪魔"),
    FlagTableRow::new(Undead, "UNDEAD", "アンデッド"),
    FlagTableRow::new(Evil, "EVIL", "邪悪"),
    FlagTableRow::new(Animal, "ANIMAL", "動物"),
    FlagTableRow::new(Amberite, "AMBERITE", "アンバーの王族"),
    FlagTableRow::new(Good, "GOOD", "善良"),
    FlagTableRow::new(Nonliving, "NONLIVING", "非生物"),
    FlagTableRow::new(Angel, "ANGEL", "天使"),
];

impl MonsterRaceFlag for MonsterKind {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_KIND_TABLE
    }
}
