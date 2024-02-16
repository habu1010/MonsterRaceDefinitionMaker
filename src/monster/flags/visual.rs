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

const MONSTER_VISUAL_TABLE: [FlagTableRow<MonsterVisual>; 6] = [
    FlagTableRow::new(Clear, "CHAR_CLEAR", "透明"),
    FlagTableRow::new(ShapeChanger, "SHAPECHANGER", "姿を変える"),
    FlagTableRow::new(ClearColor, "ATTR_CLEAR", "透明"),
    FlagTableRow::new(MultiColor, "ATTR_MULTI", "万色"),
    FlagTableRow::new(RandomColor, "ATTR_SEMIRAND", "準ランダム色"),
    FlagTableRow::new(AnyColor, "ATTR_ANY", "任意色"),
];

impl MonsterRaceFlag for MonsterVisual {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_VISUAL_TABLE
    }
}
