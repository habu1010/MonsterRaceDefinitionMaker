use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillMisc {
    Shriek,
    Scare,
    Blind,
    Confusion,
    Slow,
    Hold,
    Haste,
    Heal,
    Invulnerability,
    Dispel,
    Blink,
    Teleport,
    TeleportTo,
    TeleportAway,
    TeleportLevel,
    World,
    Darkness,
    Traps,
    Forget,
    RaiseDead,
    Special,
}

use MonsterSkillMisc::*;

pub const MONSTER_SKILL_MISC_TABLES: [FlagTable<MonsterSkillMisc>; 21] = [
    FlagTable::new(Shriek, "SHRIEK", "悲鳴を上げる"),
    FlagTable::new(Scare, "SCARE", "恐怖"),
    FlagTable::new(Blind, "BLIND", "目くらまし"),
    FlagTable::new(Confusion, "CONF", "混乱"),
    FlagTable::new(Slow, "SLOW", "減速"),
    FlagTable::new(Hold, "HOLD", "麻痺"),
    FlagTable::new(Haste, "HASTE", "加速"),
    FlagTable::new(Heal, "HEAL", "治癒"),
    FlagTable::new(Invulnerability, "INVULNER", "無敵化"),
    FlagTable::new(Dispel, "DISPEL", "魔法消去"),
    FlagTable::new(Blink, "BLINK", "ショートテレポート"),
    FlagTable::new(Teleport, "TPORT", "テレポート"),
    FlagTable::new(TeleportTo, "TELE_TO", "テレポートバック"),
    FlagTable::new(TeleportAway, "TELE_AWAY", "テレポートアウェイ"),
    FlagTable::new(TeleportLevel, "TELE_LEVEL", "テレポート・レベル"),
    FlagTable::new(World, "WORLD", "時を止める"),
    FlagTable::new(Darkness, "DARKNESS", "暗闇"),
    FlagTable::new(Traps, "TRAPS", "トラップ"),
    FlagTable::new(Forget, "FORGET", "記憶消去"),
    FlagTable::new(RaiseDead, "ANIM_DEAD", "死者復活"),
    FlagTable::new(Special, "SPECIAL", "特別な行動をする"),
];

impl MonsterRaceFlag for MonsterSkillMisc {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_SKILL_MISC_TABLES
    }
}
