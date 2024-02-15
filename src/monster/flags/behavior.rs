use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterBehavior {
    NeverBlow,
    NeverMove,
    OpenDoor,
    BashDoor,
    MoveBody,
    KillBody,
    TakeItem,
    KillItem,
    Rand25,
    Rand50,
    Stupid,
    Smart,
    Friendly,
    PreventSuddenMagic,
}

use MonsterBehavior::*;

pub const MONSTER_BEHAVIOR_TABLES: [FlagTable<MonsterBehavior>; 14] = [
    FlagTable::new(NeverBlow, "NEVER_BLOW", "物理的な攻撃手段を持たない"),
    FlagTable::new(NeverMove, "NEVER_MOVE", "移動しない"),
    FlagTable::new(OpenDoor, "OPEN_DOOR", "ドアを開ける"),
    FlagTable::new(BashDoor, "BASH_DOOR", "ドアを打ち破る"),
    FlagTable::new(MoveBody, "MOVE_BODY", "弱いモンスターを押しのける"),
    FlagTable::new(KillBody, "KILL_BODY", "弱いモンスターを倒す"),
    FlagTable::new(TakeItem, "TAKE_ITEM", "アイテムを拾う"),
    FlagTable::new(KillItem, "KILL_ITEM", "アイテムを壊す"),
    FlagTable::new(Rand25, "RAND_25", "不規則に移動(25%)"),
    FlagTable::new(Rand50, "RAND_50", "不規則に移動(50%)"),
    FlagTable::new(Stupid, "STUPID", "愚かな行動をする"),
    FlagTable::new(Smart, "SMART", "的確に行動する"),
    FlagTable::new(Friendly, "FRIENDLY", "友好的"),
    FlagTable::new(
        PreventSuddenMagic,
        "PREVENT_SUDDEN_MAGIC",
        "突然の魔法の使用をしない",
    ),
];

impl MonsterRaceFlag for MonsterBehavior {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_BEHAVIOR_TABLES
    }
}
