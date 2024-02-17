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
    Charge,
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

const MONSTER_BLOW_METHOD_TABLE: [FlagTableRow<MonsterBrowMethod>; 25] = [
    FlagTableRow::new(None, "", "(なし)"),
    FlagTableRow::new(Hit, "HIT", "殴る"),
    FlagTableRow::new(Touch, "TOUCH", "触れる"),
    FlagTableRow::new(Punch, "PUNCH", "パンチする"),
    FlagTableRow::new(Kick, "KICK", "蹴る"),
    FlagTableRow::new(Claw, "CLAW", "ひっかく"),
    FlagTableRow::new(Bite, "BITE", "噛む"),
    FlagTableRow::new(Sting, "STING", "刺す"),
    FlagTableRow::new(Slash, "SLASH", "斬る"),
    FlagTableRow::new(Butt, "BUTT", "角で突く"),
    FlagTableRow::new(Crush, "CRUSH", "体当たりする"),
    FlagTableRow::new(Engulf, "ENGULF", "飲み込む"),
    FlagTableRow::new(Charge, "CHARGE", "請求書をよこす"),
    FlagTableRow::new(Crawl, "CRAWL", "体の上を這い回る"),
    FlagTableRow::new(Drool, "DROOL", "よだれを垂らす"),
    FlagTableRow::new(Spit, "SPIT", "つばを吐く"),
    FlagTableRow::new(Explode, "EXPLODE", "爆発する"),
    FlagTableRow::new(Gaze, "GAZE", "にらむ"),
    FlagTableRow::new(Wail, "WAIL", "泣き叫ぶ"),
    FlagTableRow::new(Spore, "SPORE", "胞子を飛ばす"),
    FlagTableRow::new(Beg, "BEG", "金をせがむ"),
    FlagTableRow::new(Insult, "INSULT", "侮辱する"),
    FlagTableRow::new(Moan, "MOAN", "うめく"),
    FlagTableRow::new(Show, "SHOW", "歌う"),
    FlagTableRow::new(Shoot, "SHOOT", "射撃する"),
];

impl MonsterRaceFlag for MonsterBrowMethod {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_BLOW_METHOD_TABLE
    }
}
