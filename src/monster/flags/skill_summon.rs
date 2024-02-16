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

const MONSTER_SKILL_SUMMON_TABLE: [FlagTableRow<MonsterSkillSummon>; 17] = [
    FlagTableRow::new(Monster, "S_MONSTER", "モンスター一体召喚"),
    FlagTableRow::new(Monsters, "S_MONSTERS", "モンスター複数召喚"),
    FlagTableRow::new(Kin, "S_KIN", "救援召喚"),
    FlagTableRow::new(Ant, "S_ANT", "アリ召喚"),
    FlagTableRow::new(Spider, "S_SPIDER", "クモ召喚"),
    FlagTableRow::new(Hound, "S_HOUND", "ハウンド召喚"),
    FlagTableRow::new(Hydra, "S_HYDRA", "ヒドラ召喚"),
    FlagTableRow::new(Angel, "S_ANGEL", "天使一体召喚"),
    FlagTableRow::new(Demon, "S_DEMON", "デーモン一体召喚"),
    FlagTableRow::new(Undead, "S_UNDEAD", "アンデッド一体召喚"),
    FlagTableRow::new(Dragon, "S_DRAGON", "ドラゴン一体召喚"),
    FlagTableRow::new(HiUndead, "S_HI_UNDEAD", "強力なアンデッド召喚"),
    FlagTableRow::new(HiDragon, "S_HI_DRAGON", "古代ドラゴン召喚"),
    FlagTableRow::new(Cyber, "S_CYBER", "サイバーデーモン召喚"),
    FlagTableRow::new(Amberites, "S_AMBERITES", "アンバーの王族召喚"),
    FlagTableRow::new(Unique, "S_UNIQUE", "ユニーク・モンスター召喚"),
    FlagTableRow::new(DeadUnique, "S_DEAD_UNIQUE", "ユニーク・モンスター口寄せ"),
];

impl MonsterRaceFlag for MonsterSkillSummon {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SKILL_SUMMON_TABLE
    }
}
