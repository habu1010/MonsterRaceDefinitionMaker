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

const MONSTER_SPEAK_TABLE: [FlagTableRow<MonsterSpeak>; 5] = [
    FlagTableRow::new(All, "SPEAK_ALL", "全て"),
    FlagTableRow::new(Battle, "SPEAK_BATTLE", "戦闘時"),
    FlagTableRow::new(Fear, "SPEAK_FEAR", "恐怖時"),
    FlagTableRow::new(Friend, "SPEAK_FRIEND", "友好時"),
    FlagTableRow::new(Death, "SPEAK_DEATH", "死亡時"),
];

impl MonsterRaceFlag for MonsterSpeak {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SPEAK_TABLE
    }
}
