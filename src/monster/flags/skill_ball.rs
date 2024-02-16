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

const MONSTER_SKILL_BALL_TABLE: [FlagTableRow<MonsterSkillBall>; 16] = [
    FlagTableRow::new(Rocket, "ROCKET", "ロケット"),
    FlagTableRow::new(Acid, "BA_ACID", "アシッド・ボール"),
    FlagTableRow::new(Elec, "BA_ELEC", "サンダー・ボール"),
    FlagTableRow::new(Fire, "BA_FIRE", "ファイア・ボール"),
    FlagTableRow::new(Cold, "BA_COLD", "アイス・ボール"),
    FlagTableRow::new(Poison, "BA_POIS", "悪臭雲"),
    FlagTableRow::new(Nether, "BA_NETH", "地獄球"),
    FlagTableRow::new(Water, "BA_WATE", "ウォーター・ボール"),
    FlagTableRow::new(Nuke, "BA_NUKE", "放射能球"),
    FlagTableRow::new(Mana, "BA_MANA", "魔力の嵐"),
    FlagTableRow::new(Dark, "BA_DARK", "暗黒の嵐"),
    FlagTableRow::new(Lite, "BA_LITE", "スターバースト"),
    FlagTableRow::new(Chaos, "BA_CHAO", "純ログルス"),
    FlagTableRow::new(Void, "BA_VOID", "虚無の嵐"),
    FlagTableRow::new(Abyss, "BA_ABYSS", "深淵の嵐"),
    FlagTableRow::new(Meteor, "BA_METEOR", "メテオスォーム"),
];

impl MonsterRaceFlag for MonsterSkillBall {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SKILL_BALL_TABLE
    }
}
