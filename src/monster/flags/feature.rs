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

const MONSTER_FEATURE_TABLE: [FlagTableRow<MonsterFeature>; 5] = [
    FlagTableRow::new(PassWall, "PASS_WALL", "壁をすり抜ける"),
    FlagTableRow::new(KillWall, "KILL_WALL", "壁を掘り進む"),
    FlagTableRow::new(Aquatic, "AQUATIC", "水棲"),
    FlagTableRow::new(CanSwim, "CAN_SWIM", "水を渡る"),
    FlagTableRow::new(CanFly, "CAN_FLY", "空を飛ぶ"),
];

impl MonsterRaceFlag for MonsterFeature {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_FEATURE_TABLE
    }
}
