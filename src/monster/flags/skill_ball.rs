use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillBall {
    Rocket,
    Nuke,
    Chaos,
    Acid,
    Elec,
    Fire,
    Cold,
    Poison,
    Nether,
    Water,
    Mana,
    Dark,
    Lite,
    Void,
    Abyss,
    Meteor,
}

use MonsterSkillBall::*;

pub const MONSTER_SKILL_BALL_TABLES: [FlagTable<MonsterSkillBall>; 16] = [
    FlagTable::new(Rocket, "ROCKET", "ロケット"),
    FlagTable::new(Acid, "BA_ACID", "アシッド・ボール"),
    FlagTable::new(Elec, "BA_ELEC", "サンダー・ボール"),
    FlagTable::new(Fire, "BA_FIRE", "ファイア・ボール"),
    FlagTable::new(Cold, "BA_COLD", "アイス・ボール"),
    FlagTable::new(Poison, "BA_POIS", "悪臭雲"),
    FlagTable::new(Nether, "BA_NETH", "地獄球"),
    FlagTable::new(Water, "BA_WATE", "ウォーター・ボール"),
    FlagTable::new(Nuke, "BA_NUKE", "放射能球"),
    FlagTable::new(Mana, "BA_MANA", "魔力の嵐"),
    FlagTable::new(Dark, "BA_DARK", "暗黒の嵐"),
    FlagTable::new(Lite, "BA_LITE", "スターバースト"),
    FlagTable::new(Chaos, "BA_CHAO", "純ログルス"),
    FlagTable::new(Void, "BA_VOID", "虚無の嵐"),
    FlagTable::new(Abyss, "BA_ABYSS", "深淵の嵐"),
    FlagTable::new(Meteor, "BA_METEOR", "メテオスォーム"),
];

impl MonsterRaceFlag for MonsterSkillBall {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_SKILL_BALL_TABLES
    }
}
