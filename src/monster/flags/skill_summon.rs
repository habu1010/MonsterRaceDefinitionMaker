use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillSummon {
    Monster,
    Monsters,
    Kin,
    Ant,
    Spider,
    Hound,
    Hydra,
    Angel,
    Demon,
    Undead,
    Dragon,
    HiUndead,
    HiDragon,
    Cyber,
    Amberites,
    Unique,
    DeadUnique,
}

use MonsterSkillSummon::*;

pub const MONSTER_SKILL_SUMMON_TABLES: [FlagTable<MonsterSkillSummon>; 17] = [
    FlagTable::new(Monster, "S_MONSTER", "モンスター一体召喚"),
    FlagTable::new(Monsters, "S_MONSTERS", "モンスター複数召喚"),
    FlagTable::new(Kin, "S_KIN", "救援召喚"),
    FlagTable::new(Ant, "S_ANT", "アリ召喚"),
    FlagTable::new(Spider, "S_SPIDER", "クモ召喚"),
    FlagTable::new(Hound, "S_HOUND", "ハウンド召喚"),
    FlagTable::new(Hydra, "S_HYDRA", "ヒドラ召喚"),
    FlagTable::new(Angel, "S_ANGEL", "天使一体召喚"),
    FlagTable::new(Demon, "S_DEMON", "デーモン一体召喚"),
    FlagTable::new(Undead, "S_UNDEAD", "アンデッド一体召喚"),
    FlagTable::new(Dragon, "S_DRAGON", "ドラゴン一体召喚"),
    FlagTable::new(HiUndead, "S_HI_UNDEAD", "強力なアンデッド召喚"),
    FlagTable::new(HiDragon, "S_HI_DRAGON", "古代ドラゴン召喚"),
    FlagTable::new(Cyber, "S_CYBER", "サイバーデーモン召喚"),
    FlagTable::new(Amberites, "S_AMBERITES", "アンバーの王族召喚"),
    FlagTable::new(Unique, "S_UNIQUE", "ユニーク・モンスター召喚"),
    FlagTable::new(DeadUnique, "S_DEAD_UNIQUE", "ユニーク・モンスター口寄せ"),
];

impl MonsterRaceFlag for MonsterSkillSummon {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_SKILL_SUMMON_TABLES
    }
}
