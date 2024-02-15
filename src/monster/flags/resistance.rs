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

pub const MONSTER_RESISTANCE_TABLES: [FlagTable<MonsterResistance>; 27] = [
    FlagTable::new(Acid, "IM_ACID", "酸"),
    FlagTable::new(Elec, "IM_ELEC", "稲妻"),
    FlagTable::new(Fire, "IM_FIRE", "炎"),
    FlagTable::new(Cold, "IM_COLD", "冷気"),
    FlagTable::new(Poison, "IM_POIS", "毒"),
    FlagTable::new(Lite, "RES_LITE", "閃光"),
    FlagTable::new(Dark, "RES_DARK", "暗黒"),
    FlagTable::new(Nether, "RES_NETH", "地獄"),
    FlagTable::new(Water, "RES_WATE", "水"),
    FlagTable::new(Plasma, "RES_PLAS", "プラズマ"),
    FlagTable::new(Shards, "RES_SHAR", "破片"),
    FlagTable::new(Sound, "RES_SOUN", "轟音"),
    FlagTable::new(Chaos, "RES_CHAO", "カオス"),
    FlagTable::new(Nexus, "RES_NEXU", "因果混乱"),
    FlagTable::new(Disenchant, "RES_DISE", "劣化"),
    FlagTable::new(Force, "RES_WALL", "フォース"),
    FlagTable::new(Inertia, "RES_INER", "遅鈍"),
    FlagTable::new(Time, "RES_TIME", "時間逆転"),
    FlagTable::new(Gravity, "RES_GRAV", "重力"),
    FlagTable::new(Meteor, "RES_METEOR", "隕石"),
    FlagTable::new(Teleport, "RES_TELE", "テレポート"),
    FlagTable::new(Fear, "RES_FEAR", "恐怖"),
    FlagTable::new(Stun, "RES_STUN", "朦朧"),
    FlagTable::new(Confusion, "RES_CONF", "混乱"),
    FlagTable::new(Sleep, "RES_SLEEP", "睡眠"),
    FlagTable::new(InstantlyDeath, "NO_INSTANTLY_DEATH", "即死"),
    FlagTable::new(All, "RES_ALL", "あらゆる攻撃"),
];

impl MonsterRaceFlag for MonsterResistance {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_RESISTANCE_TABLES
    }
}
