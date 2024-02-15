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

const MONSTER_SEX_TABLES: [FlagTable<MonsterSex>; 3] = [
    FlagTable::new(None, "", "区別なし"),
    FlagTable::new(Male, "MALE", "男性/雄"),
    FlagTable::new(Female, "FEMALE", "女性/雌"),
];

impl MonsterRaceFlag for MonsterSex {
    fn get_flag_tables(&self) -> &'static [FlagTable<Self>] {
        &MONSTER_SEX_TABLES
    }
}
