use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterVisual {
    Clear,
    ShapeChanger,
    ClearColor,
    MultiColor,
    RandomColor,
    AnyColor,
}

use MonsterVisual::*;

pub const MONSTER_VISUAL_TABLES: [FlagTable<MonsterVisual>; 6] = [
    FlagTable::new(Clear, "CHAR_CLEAR", "透明"),
    FlagTable::new(ShapeChanger, "SHAPECHANGER", "姿を変える"),
    FlagTable::new(ClearColor, "ATTR_CLEAR", "透明"),
    FlagTable::new(MultiColor, "ATTR_MULTI", "万色"),
    FlagTable::new(RandomColor, "ATTR_SEMIRAND", "準ランダム色"),
    FlagTable::new(AnyColor, "ATTR_ANY", "任意色"),
];

impl MonsterRaceFlag for MonsterVisual {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_VISUAL_TABLES
    }
}
