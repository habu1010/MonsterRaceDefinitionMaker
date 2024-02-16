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

const MONSTER_MISC_TABLE: [FlagTableRow<MonsterMisc>; 20] = [
    FlagTableRow::new(ForceDepth, "FORCE_DEPTH", "階層固定"),
    FlagTableRow::new(ForceMaxHp, "FORCE_MAXHP", "最大HP固定"),
    FlagTableRow::new(HasFriends, "HAS_FRIENDS", "集団で現れる"),
    FlagTableRow::new(Escort, "ESCORT", "護衛を伴って現れる"),
    FlagTableRow::new(MoreEscort, "MORE_ESCORT", "多くの護衛を伴って現れる"),
    FlagTableRow::new(Riding, "RIDING", "騎乗できる"),
    FlagTableRow::new(Invisible, "INVISIBLE", "透明で目に見えない"),
    FlagTableRow::new(ColdBlood, "COLD_BLOOD", "冷血動物"),
    FlagTableRow::new(Kage, "KAGE", "あやしい影"),
    FlagTableRow::new(Chameleon, "CHAMELEON", "カメレオン"),
    FlagTableRow::new(Tanuki, "TANUKI", "たぬき"),
    FlagTableRow::new(NoQuest, "NO_QUEST", "クエスト対象にならない"),
    FlagTableRow::new(EldritchHorror, "ELDRITCH_HORROR", "狂気を誘う"),
    FlagTableRow::new(Multiply, "MULTIPLY", "爆発的に増殖する"),
    FlagTableRow::new(Regenerate, "REGENERATE", "素早く体力を回復する"),
    FlagTableRow::new(Powerful, "POWERFUL", "強力な魔法/スキル"),
    FlagTableRow::new(Reflecting, "REFLECTING", "矢の呪文を跳ね返す"),
    FlagTableRow::new(Questor, "QUESTOR", "クエスト専用モンスター"),
    FlagTableRow::new(EmptyMind, "EMPTY_MIND", "テレパシーで感知できない"),
    FlagTableRow::new(WiredMind, "WIRED_MIND", "まれにテレパシーで感知できる"),
];

impl MonsterRaceFlag for MonsterMisc {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_MISC_TABLE
    }
}
