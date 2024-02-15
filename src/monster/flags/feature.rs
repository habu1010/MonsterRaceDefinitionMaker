use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterFeature {
    PassWall,
    KillWall,
    Aquatic,
    CanSwim,
    CanFly,
}

use MonsterFeature::*;

pub const MONSTER_FEATURE_TABLES: [FlagTable<MonsterFeature>; 5] = [
    FlagTable::new(PassWall, "PASS_WALL", "壁をすり抜ける"),
    FlagTable::new(KillWall, "KILL_WALL", "壁を掘り進む"),
    FlagTable::new(Aquatic, "AQUATIC", "水棲"),
    FlagTable::new(CanSwim, "CAN_SWIM", "水を渡る"),
    FlagTable::new(CanFly, "CAN_FLY", "空を飛ぶ"),
];

impl MonsterRaceFlag for MonsterFeature {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_FEATURE_TABLES
    }
}
