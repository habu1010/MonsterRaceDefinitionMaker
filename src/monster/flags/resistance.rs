use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterResistance {
    Acid,
    Elec,
    Fire,
    Cold,
    Poison,
    Lite,
    Dark,
    Nether,
    Water,
    Plasma,
    Shards,
    Sound,
    Chaos,
    Nexus,
    Disenchant,
    Force,
    Inertia,
    Time,
    Gravity,
    Meteor,
    Teleport,
    Fear,
    Stun,
    Confusion,
    Sleep,
    InstantlyDeath,
    All,
}

use MonsterResistance::*;

const MONSTER_RESISTANCE_TABLE: [FlagTableRow<MonsterResistance>; 27] = [
    FlagTableRow::new(Acid, "IM_ACID", "酸"),
    FlagTableRow::new(Elec, "IM_ELEC", "稲妻"),
    FlagTableRow::new(Fire, "IM_FIRE", "炎"),
    FlagTableRow::new(Cold, "IM_COLD", "冷気"),
    FlagTableRow::new(Poison, "IM_POIS", "毒"),
    FlagTableRow::new(Lite, "RES_LITE", "閃光"),
    FlagTableRow::new(Dark, "RES_DARK", "暗黒"),
    FlagTableRow::new(Nether, "RES_NETH", "地獄"),
    FlagTableRow::new(Water, "RES_WATE", "水"),
    FlagTableRow::new(Plasma, "RES_PLAS", "プラズマ"),
    FlagTableRow::new(Shards, "RES_SHAR", "破片"),
    FlagTableRow::new(Sound, "RES_SOUN", "轟音"),
    FlagTableRow::new(Chaos, "RES_CHAO", "カオス"),
    FlagTableRow::new(Nexus, "RES_NEXU", "因果混乱"),
    FlagTableRow::new(Disenchant, "RES_DISE", "劣化"),
    FlagTableRow::new(Force, "RES_WALL", "フォース"),
    FlagTableRow::new(Inertia, "RES_INER", "遅鈍"),
    FlagTableRow::new(Time, "RES_TIME", "時間逆転"),
    FlagTableRow::new(Gravity, "RES_GRAV", "重力"),
    FlagTableRow::new(Meteor, "RES_METEOR", "隕石"),
    FlagTableRow::new(Teleport, "RES_TELE", "テレポート"),
    FlagTableRow::new(Fear, "RES_FEAR", "恐怖"),
    FlagTableRow::new(Stun, "RES_STUN", "朦朧"),
    FlagTableRow::new(Confusion, "RES_CONF", "混乱"),
    FlagTableRow::new(Sleep, "RES_SLEEP", "睡眠"),
    FlagTableRow::new(InstantlyDeath, "NO_INSTANTLY_DEATH", "即死"),
    FlagTableRow::new(All, "RES_ALL", "あらゆる攻撃"),
];

impl MonsterRaceFlag for MonsterResistance {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_RESISTANCE_TABLE
    }
}
