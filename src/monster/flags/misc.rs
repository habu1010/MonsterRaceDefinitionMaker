use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterMisc {
    ForceDepth,
    ForceMaxHp,
    HasFriends,
    Escort,
    MoreEscort,
    Riding,
    Invisible,
    ColdBlood,
    Kage,
    Chameleon,
    Tanuki,
    NoQuest,
    EldritchHorror,
    Multiply,
    Regenerate,
    Powerful,
    Reflecting,
    Questor,
    EmptyMind,
    WiredMind,
}

use MonsterMisc::*;

pub const MONSTER_MISC_TABLES: [FlagTable<MonsterMisc>; 20] = [
    FlagTable::new(ForceDepth, "FORCE_DEPTH", "階層固定"),
    FlagTable::new(ForceMaxHp, "FORCE_MAXHP", "最大HP固定"),
    FlagTable::new(HasFriends, "HAS_FRIENDS", "集団で現れる"),
    FlagTable::new(Escort, "ESCORT", "護衛を伴って現れる"),
    FlagTable::new(MoreEscort, "MORE_ESCORT", "多くの護衛を伴って現れる"),
    FlagTable::new(Riding, "RIDING", "騎乗できる"),
    FlagTable::new(Invisible, "INVISIBLE", "透明で目に見えない"),
    FlagTable::new(ColdBlood, "COLD_BLOOD", "冷血動物"),
    FlagTable::new(Kage, "KAGE", "あやしい影"),
    FlagTable::new(Chameleon, "CHAMELEON", "カメレオン"),
    FlagTable::new(Tanuki, "TANUKI", "たぬき"),
    FlagTable::new(NoQuest, "NO_QUEST", "クエスト対象にならない"),
    FlagTable::new(EldritchHorror, "ELDRITCH_HORROR", "狂気を誘う"),
    FlagTable::new(Multiply, "MULTIPLY", "爆発的に増殖する"),
    FlagTable::new(Regenerate, "REGENERATE", "素早く体力を回復する"),
    FlagTable::new(Powerful, "POWERFUL", "強力な魔法/スキル"),
    FlagTable::new(Reflecting, "REFLECTING", "矢の呪文を跳ね返す"),
    FlagTable::new(Questor, "QUESTOR", "クエスト専用モンスター"),
    FlagTable::new(EmptyMind, "EMPTY_MIND", "テレパシーで感知できない"),
    FlagTable::new(WiredMind, "WIRED_MIND", "まれにテレパシーで感知できる"),
];

impl MonsterRaceFlag for MonsterMisc {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_MISC_TABLES
    }
}
