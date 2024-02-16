use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterWildness {
    WildOnly,
    Town,
    Shore,
    Ocean,
    Waste,
    Wood,
    Volcano,
    Mountain,
    Grass,
    All,
}

use MonsterWildness::*;

const MONSTER_WILDNESS_TABLE: [FlagTableRow<MonsterWildness>; 10] = [
    FlagTableRow::new(WildOnly, "WILD_ONLY", "地上のみ"),
    FlagTableRow::new(Town, "WILD_TOWN", "街"),
    FlagTableRow::new(Shore, "WILD_SHORE", "浅瀬"),
    FlagTableRow::new(Ocean, "WILD_OCEAN", "海"),
    FlagTableRow::new(Waste, "WILD_WASTE", "荒野"),
    FlagTableRow::new(Wood, "WILD_WOOD", "森林"),
    FlagTableRow::new(Volcano, "WILD_VOLCANO", "火山"),
    FlagTableRow::new(Mountain, "WILD_MOUNTAIN", "山"),
    FlagTableRow::new(Grass, "WILD_GRASS", "草原"),
    FlagTableRow::new(All, "WILD_ALL", "全て"),
];

impl MonsterRaceFlag for MonsterWildness {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_WILDNESS_TABLE
    }
}
