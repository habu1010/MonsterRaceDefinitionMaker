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

const MONSTER_SKILL_MISC_TABLE: [FlagTableRow<MonsterSkillMisc>; 21] = [
    FlagTableRow::new(Shriek, "SHRIEK", "悲鳴を上げる"),
    FlagTableRow::new(Scare, "SCARE", "恐怖"),
    FlagTableRow::new(Blind, "BLIND", "目くらまし"),
    FlagTableRow::new(Confusion, "CONF", "混乱"),
    FlagTableRow::new(Slow, "SLOW", "減速"),
    FlagTableRow::new(Hold, "HOLD", "麻痺"),
    FlagTableRow::new(Haste, "HASTE", "加速"),
    FlagTableRow::new(Heal, "HEAL", "治癒"),
    FlagTableRow::new(Invulnerability, "INVULNER", "無敵化"),
    FlagTableRow::new(Dispel, "DISPEL", "魔法消去"),
    FlagTableRow::new(Blink, "BLINK", "ショートテレポート"),
    FlagTableRow::new(Teleport, "TPORT", "テレポート"),
    FlagTableRow::new(TeleportTo, "TELE_TO", "テレポートバック"),
    FlagTableRow::new(TeleportAway, "TELE_AWAY", "テレポートアウェイ"),
    FlagTableRow::new(TeleportLevel, "TELE_LEVEL", "テレポート・レベル"),
    FlagTableRow::new(World, "WORLD", "時を止める"),
    FlagTableRow::new(Darkness, "DARKNESS", "暗闇"),
    FlagTableRow::new(Traps, "TRAPS", "トラップ"),
    FlagTableRow::new(Forget, "FORGET", "記憶消去"),
    FlagTableRow::new(RaiseDead, "ANIM_DEAD", "死者復活"),
    FlagTableRow::new(Special, "SPECIAL", "特別な行動をする"),
];

impl MonsterRaceFlag for MonsterSkillMisc {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SKILL_MISC_TABLE
    }
}
