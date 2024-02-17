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

const MONSTER_DROP_TABLE: [FlagTableRow<MonsterDrop>; 12] = [
    FlagTableRow::new(OnlyGold, "ONLY_GOLD", "金のみ"),
    FlagTableRow::new(OnlyItem, "ONLY_ITEM", "アイテムのみ"),
    FlagTableRow::new(Good, "DROP_GOOD", "上質なアイテム"),
    FlagTableRow::new(Great, "DROP_GREAT", "特別なアイテム"),
    FlagTableRow::new(Corpse, "DROP_CORPSE", "死体を残す"),
    FlagTableRow::new(Skeleton, "DROP_SKELETON", "骨を残す"),
    FlagTableRow::new(Drop60, "DROP_60", "60%"),
    FlagTableRow::new(Drop90, "DROP_90", "90%"),
    FlagTableRow::new(Drop1D2, "DROP_1D2", "1D2 個"),
    FlagTableRow::new(Drop2D2, "DROP_2D2", "2D2 個"),
    FlagTableRow::new(Drop3D2, "DROP_3D2", "3D2 個"),
    FlagTableRow::new(Drop4D2, "DROP_4D2", "4D2 個"),
];

impl MonsterRaceFlag for MonsterDrop {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_DROP_TABLE
    }
}
