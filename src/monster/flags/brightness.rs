use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterBrightness {
    HasLite1,
    SelfLite1,
    HasLite2,
    SelfLite2,
    HasDark1,
    SelfDark1,
    HasDark2,
    SelfDark2,
}

use MonsterBrightness::*;

const MONSTER_BRIGHTNESS_TABLE: [FlagTableRow<MonsterBrightness>; 8] = [
    FlagTableRow::new(HasLite1, "HAS_LITE_1", "ダンジョンを照らす(半径1)"),
    FlagTableRow::new(SelfLite1, "SELF_LITE_1", "光っている(半径1)"),
    FlagTableRow::new(HasLite2, "HAS_LITE_2", "ダンジョンを照らす(半径2)"),
    FlagTableRow::new(SelfLite2, "SELF_LITE_2", "光っている(半径2)"),
    FlagTableRow::new(HasDark1, "HAS_DARK_1", "ダンジョンを暗くする(半径1)"),
    FlagTableRow::new(SelfDark1, "SELF_DARK_1", "暗闇に包まれている(半径1)"),
    FlagTableRow::new(HasDark2, "HAS_DARK_2", "ダンジョンを暗くする(半径2)"),
    FlagTableRow::new(SelfDark2, "SELF_DARK_2", "暗闇に包まれている(半径2)"),
];

impl MonsterRaceFlag for MonsterBrightness {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_BRIGHTNESS_TABLE
    }
}
