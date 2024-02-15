use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSpeak {
    All,
    Battle,
    Fear,
    Friend,
    Death,
    Spawn,
}

use MonsterSpeak::*;

pub const MONSTER_SPEAK_TABLES: [FlagTable<MonsterSpeak>; 5] = [
    FlagTable::new(All, "SPEAK_ALL", "全て"),
    FlagTable::new(Battle, "SPEAK_BATTLE", "戦闘時"),
    FlagTable::new(Fear, "SPEAK_FEAR", "恐怖時"),
    FlagTable::new(Friend, "SPEAK_FRIEND", "友好時"),
    FlagTable::new(Death, "SPEAK_DEATH", "死亡時"),
];

impl MonsterRaceFlag for MonsterSpeak {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_SPEAK_TABLES
    }
}
