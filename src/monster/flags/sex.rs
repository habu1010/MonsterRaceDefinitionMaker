use super::*;

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterSex {
    #[default]
    None,
    Male,
    Female,
}

use MonsterSex::*;

const MONSTER_SEX_TABLE: [FlagTableRow<MonsterSex>; 3] = [
    FlagTableRow::new(None, "", "区別なし"),
    FlagTableRow::new(Male, "MALE", "男性/雄"),
    FlagTableRow::new(Female, "FEMALE", "女性/雌"),
];

impl MonsterRaceFlag for MonsterSex {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SEX_TABLE
    }
}
