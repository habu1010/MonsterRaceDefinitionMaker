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

const MONSTER_BEHAVIOR_TABLE: [FlagTableRow<MonsterBehavior>; 14] = [
    FlagTableRow::new(NeverBlow, "NEVER_BLOW", "物理的な攻撃手段を持たない"),
    FlagTableRow::new(NeverMove, "NEVER_MOVE", "移動しない"),
    FlagTableRow::new(OpenDoor, "OPEN_DOOR", "ドアを開ける"),
    FlagTableRow::new(BashDoor, "BASH_DOOR", "ドアを打ち破る"),
    FlagTableRow::new(MoveBody, "MOVE_BODY", "弱いモンスターを押しのける"),
    FlagTableRow::new(KillBody, "KILL_BODY", "弱いモンスターを倒す"),
    FlagTableRow::new(TakeItem, "TAKE_ITEM", "アイテムを拾う"),
    FlagTableRow::new(KillItem, "KILL_ITEM", "アイテムを壊す"),
    FlagTableRow::new(Rand25, "RAND_25", "不規則に移動(25%)"),
    FlagTableRow::new(Rand50, "RAND_50", "不規則に移動(50%)"),
    FlagTableRow::new(Stupid, "STUPID", "愚かな行動をする"),
    FlagTableRow::new(Smart, "SMART", "的確に行動する"),
    FlagTableRow::new(Friendly, "FRIENDLY", "友好的"),
    FlagTableRow::new(
        PreventSuddenMagic,
        "PREVENT_SUDDEN_MAGIC",
        "突然の魔法の使用をしない",
    ),
];

impl MonsterRaceFlag for MonsterBehavior {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_BEHAVIOR_TABLE
    }
}
