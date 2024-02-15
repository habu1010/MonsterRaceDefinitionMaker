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

pub const MONSTER_BRIGHTNESS_TABLES: [FlagTable<MonsterBrightness>; 8] = [
    FlagTable::new(HasLite1, "HAS_LITE1", "ダンジョンを照らす(半径1)"),
    FlagTable::new(SelfLite1, "SELF_LITE1", "光っている(半径1)"),
    FlagTable::new(HasLite2, "HAS_LITE2", "ダンジョンを照らす(半径2)"),
    FlagTable::new(SelfLite2, "SELF_LITE2", "光っている(半径2)"),
    FlagTable::new(HasDark1, "HAS_DARK1", "ダンジョンを暗くする(半径1)"),
    FlagTable::new(SelfDark1, "SELF_DARK1", "暗闇に包まれている(半径1)"),
    FlagTable::new(HasDark2, "HAS_DARK2", "ダンジョンを暗くする(半径2)"),
    FlagTable::new(SelfDark2, "SELF_DARK2", "暗闇に包まれている(半径2)"),
];

impl MonsterRaceFlag for MonsterBrightness {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_BRIGHTNESS_TABLES
    }
}
