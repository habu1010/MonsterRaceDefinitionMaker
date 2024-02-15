use super::*;

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterBrowMethod {
    #[default]
    None,
    Hit,
    Touch,
    Punch,
    Kick,
    Claw,
    Bite,
    Sting,
    Slash,
    Butt,
    Crush,
    Engulf,
    Change,
    Crawl,
    Drool,
    Spit,
    Explode,
    Gaze,
    Wail,
    Spore,
    Beg,
    Insult,
    Moan,
    Show,
    Shoot,
}

use MonsterBrowMethod::*;

pub const MONSTER_BLOW_METHOD_TABLES: [FlagTable<MonsterBrowMethod>; 25] = [
    FlagTable::new(None, "", "(なし)"),
    FlagTable::new(Hit, "HIT", "殴る"),
    FlagTable::new(Touch, "TOUCH", "触れる"),
    FlagTable::new(Punch, "PUNCH", "パンチする"),
    FlagTable::new(Kick, "KICK", "蹴る"),
    FlagTable::new(Claw, "CLAW", "ひっかく"),
    FlagTable::new(Bite, "BITE", "噛む"),
    FlagTable::new(Sting, "STING", "刺す"),
    FlagTable::new(Slash, "SLASH", "斬る"),
    FlagTable::new(Butt, "BUTT", "角で突く"),
    FlagTable::new(Crush, "CRUSH", "体当たりする"),
    FlagTable::new(Engulf, "ENGULF", "飲み込む"),
    FlagTable::new(Change, "CHANGE", "請求書をよこす"),
    FlagTable::new(Crawl, "CRAWL", "体の上を這い回る"),
    FlagTable::new(Drool, "DROOL", "よだれを垂らす"),
    FlagTable::new(Spit, "SPIT", "つばを吐く"),
    FlagTable::new(Explode, "EXPLODE", "爆発する"),
    FlagTable::new(Gaze, "GAZE", "にらむ"),
    FlagTable::new(Wail, "WAIL", "泣き叫ぶ"),
    FlagTable::new(Spore, "SPORE", "胞子を飛ばす"),
    FlagTable::new(Beg, "BEG", "金をせがむ"),
    FlagTable::new(Insult, "INSULT", "侮辱する"),
    FlagTable::new(Moan, "MOAN", "うめく"),
    FlagTable::new(Show, "SHOW", "歌う"),
    FlagTable::new(Shoot, "SHOOT", "射撃する"),
];

impl MonsterRaceFlag for MonsterBrowMethod {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_BLOW_METHOD_TABLES
    }
}
