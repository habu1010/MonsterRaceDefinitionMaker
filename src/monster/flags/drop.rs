use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterDrop {
    OnlyGold,
    OnlyItem,
    Good,
    Great,
    Corpse,
    Skeleton,
    Drop60,
    Drop90,
    Drop1D2,
    Drop2D2,
    Drop3D2,
    Drop4D2,
}

use MonsterDrop::*;

pub const MONSTER_DROP_TABLES: [FlagTable<MonsterDrop>; 12] = [
    FlagTable::new(OnlyGold, "DROP_GOLD", "金のみ"),
    FlagTable::new(OnlyItem, "DROP_ITEM", "アイテムのみ"),
    FlagTable::new(Good, "DROP_GOOD", "上質なアイテム"),
    FlagTable::new(Great, "DROP_GREAT", "特別なアイテム"),
    FlagTable::new(Corpse, "DROP_CORPSE", "死体を残す"),
    FlagTable::new(Skeleton, "DROP_SKELETON", "骨を残す"),
    FlagTable::new(Drop60, "DROP_60", "60%"),
    FlagTable::new(Drop90, "DROP_90", "90%"),
    FlagTable::new(Drop1D2, "DROP_1D2", "1D2 個"),
    FlagTable::new(Drop2D2, "DROP_2D2", "2D2 個"),
    FlagTable::new(Drop3D2, "DROP_3D2", "3D2 個"),
    FlagTable::new(Drop4D2, "DROP_4D2", "4D2 個"),
];

impl MonsterRaceFlag for MonsterDrop {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_DROP_TABLES
    }
}
