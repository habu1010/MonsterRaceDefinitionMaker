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

pub const MONSTER_WILDNESS_TABLES: [FlagTable<MonsterWildness>; 10] = [
    FlagTable::new(WildOnly, "WILD_ONLY", "地上のみ"),
    FlagTable::new(Town, "WILD_TOWN", "街"),
    FlagTable::new(Shore, "WILD_SHORE", "浅瀬"),
    FlagTable::new(Ocean, "WILD_OCEAN", "海"),
    FlagTable::new(Waste, "WILD_WASTE", "荒野"),
    FlagTable::new(Wood, "WILD_WOOD", "森林"),
    FlagTable::new(Volcano, "WILD_VOLCANO", "火山"),
    FlagTable::new(Mountain, "WILD_MOUNTAIN", "山"),
    FlagTable::new(Grass, "WILD_GRASS", "草原"),
    FlagTable::new(All, "WILD_ALL", "全て"),
];

impl MonsterRaceFlag for MonsterWildness {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_WILDNESS_TABLES
    }
}
