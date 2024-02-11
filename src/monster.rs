#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterRace {
    pub name: String,
    pub english_name: String,

    pub symbol: MonsterSymbol,

    pub hp: MonsterHitDice,
    pub speed: i32,
    pub vision: i32,
    pub ac: i32,
    pub alertness: i32,

    pub level: i32,
    pub rarity: i32,
    pub exp: i32,
    pub evolving_exp: i32,
    pub evolves_to: i32,

    pub sex: MonsterSex,

    pub blows: [MonsterBlow; 4],
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterSymbol {
    pub char: String,
    pub color: crate::color::Color,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterHitDice {
    pub num: i32,
    pub sides: i32,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterBlow {
    pub method: MonsterBrowMethod,
    pub effect: MonsterBrowEffect,
    pub has_damage: bool,
    pub damage_dice: MonsterHitDice,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum MonsterSex {
    #[default]
    None,
    Male,
    Female,
}

pub struct FlagTable<T> {
    pub flag: T,
    pub flag_str: &'static str,
    pub description: &'static str,
}

impl<T> FlagTable<T> {
    const fn new(flag: T, flag_str: &'static str, description: &'static str) -> Self {
        Self {
            flag,
            flag_str,
            description,
        }
    }
}

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

use MonsterBrowMethod as Mbm;

pub const MONSTER_BLOW_METHOD_TABLES: [FlagTable<MonsterBrowMethod>; 25] = [
    FlagTable::new(Mbm::None, "", "(なし)"),
    FlagTable::new(Mbm::Hit, "HIT", "殴る"),
    FlagTable::new(Mbm::Touch, "TOUCH", "触れる"),
    FlagTable::new(Mbm::Punch, "PUNCH", "パンチする"),
    FlagTable::new(Mbm::Kick, "KICK", "蹴る"),
    FlagTable::new(Mbm::Claw, "CLAW", "ひっかく"),
    FlagTable::new(Mbm::Bite, "BITE", "噛む"),
    FlagTable::new(Mbm::Sting, "STING", "刺す"),
    FlagTable::new(Mbm::Slash, "SLASH", "斬る"),
    FlagTable::new(Mbm::Butt, "BUTT", "角で突く"),
    FlagTable::new(Mbm::Crush, "CRUSH", "体当たりする"),
    FlagTable::new(Mbm::Engulf, "ENGULF", "飲み込む"),
    FlagTable::new(Mbm::Change, "CHANGE", "請求書をよこす"),
    FlagTable::new(Mbm::Crawl, "CRAWL", "体の上を這い回る"),
    FlagTable::new(Mbm::Drool, "DROOL", "よだれを垂らす"),
    FlagTable::new(Mbm::Spit, "SPIT", "つばを吐く"),
    FlagTable::new(Mbm::Explode, "EXPLODE", "爆発する"),
    FlagTable::new(Mbm::Gaze, "GAZE", "にらむ"),
    FlagTable::new(Mbm::Wail, "WAIL", "泣き叫ぶ"),
    FlagTable::new(Mbm::Spore, "SPORE", "胞子を飛ばす"),
    FlagTable::new(Mbm::Beg, "BEG", "金をせがむ"),
    FlagTable::new(Mbm::Insult, "INSULT", "侮辱する"),
    FlagTable::new(Mbm::Moan, "MOAN", "うめく"),
    FlagTable::new(Mbm::Show, "SHOW", "歌う"),
    FlagTable::new(Mbm::Shoot, "SHOOT", "射撃する"),
];

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterBrowEffect {
    #[default]
    None,
    Hurt,
    SuperHurt,
    Poison,
    UnBonus,
    UnPower,
    EatGold,
    EatItem,
    EatFood,
    EatLite,
    Acid,
    Elec,
    Fire,
    Cold,
    Blind,
    Confuse,
    Terrify,
    Paralyze,
    LoseStr,
    LoseInt,
    LoseWis,
    LoseDex,
    LoseCon,
    LoseChr,
    LoseAll,
    Shatter,
    Exp10,
    Exp20,
    Exp40,
    Exp80,
    Disease,
    Time,
    DrainLife,
    DrainMana,
    Inertia,
    Stun,
    Hungry,
    Chaos,
    Flavor,
}

use MonsterBrowEffect as Mbe;

pub const MONSTER_BLOW_EFFECT_TABLES: [FlagTable<MonsterBrowEffect>; 39] = [
    FlagTable::new(Mbe::None, "FLAVOR", "(なし)"), // FLAVORと同じ扱いにする
    FlagTable::new(Mbe::Hurt, "HURT", "攻撃する"),
    FlagTable::new(Mbe::SuperHurt, "SUPERHURT", "強力に攻撃する"),
    FlagTable::new(Mbe::Poison, "POISON", "毒をくらわす"),
    FlagTable::new(Mbe::UnBonus, "UN_BONUS", "劣化させる"),
    FlagTable::new(Mbe::UnPower, "UN_POWER", "充填魔力を吸収する"),
    FlagTable::new(Mbe::EatGold, "EAT_GOLD", "金を盗む"),
    FlagTable::new(Mbe::EatItem, "EAT_ITEM", "アイテムを盗む"),
    FlagTable::new(Mbe::EatFood, "EAT_FOOD", "あなたの食料を食べる"),
    FlagTable::new(Mbe::EatLite, "EAT_LITE", "明かりを吸収する"),
    FlagTable::new(Mbe::Acid, "ACID", "酸を飛ばす"),
    FlagTable::new(Mbe::Elec, "ELEC", "感電させる"),
    FlagTable::new(Mbe::Fire, "FIRE", "燃やす"),
    FlagTable::new(Mbe::Cold, "COLD", "凍らせる"),
    FlagTable::new(Mbe::Blind, "BLIND", "盲目にする"),
    FlagTable::new(Mbe::Confuse, "CONFUSE", "混乱させる"),
    FlagTable::new(Mbe::Terrify, "TERRIFY", "恐怖させる"),
    FlagTable::new(Mbe::Paralyze, "PARALYZE", "麻痺させる"),
    FlagTable::new(Mbe::LoseStr, "LOSE_STR", "筋力を減少させる"),
    FlagTable::new(Mbe::LoseInt, "LOSE_INT", "知能を減少させる"),
    FlagTable::new(Mbe::LoseWis, "LOSE_WIS", "賢さを減少させる"),
    FlagTable::new(Mbe::LoseDex, "LOSE_DEX", "器用さを減少させる"),
    FlagTable::new(Mbe::LoseCon, "LOSE_CON", "耐久力を減少させる"),
    FlagTable::new(Mbe::LoseChr, "LOSE_CHR", "魅力を減少させる"),
    FlagTable::new(Mbe::LoseAll, "LOSE_ALL", "全ステータスを減少させる"),
    FlagTable::new(Mbe::Shatter, "SHATTER", "粉砕する"),
    FlagTable::new(Mbe::Exp10, "EXP_10", "経験値を減少(10d6+)させる"),
    FlagTable::new(Mbe::Exp20, "EXP_20", "経験値を減少(20d6+)させる"),
    FlagTable::new(Mbe::Exp40, "EXP_40", "経験値を減少(40d6+)させる"),
    FlagTable::new(Mbe::Exp80, "EXP_80", "経験値を減少(80d6+)させる"),
    FlagTable::new(Mbe::Disease, "DISEASE", "病気にする"),
    FlagTable::new(Mbe::Time, "TIME", "時間を逆戻りさせる"),
    FlagTable::new(Mbe::DrainLife, "EXP_VAMP", "生命力を吸収する"),
    FlagTable::new(Mbe::DrainMana, "DR_MANA", "魔力を奪う"),
    FlagTable::new(Mbe::Inertia, "INERTIA", "減速させる"),
    FlagTable::new(Mbe::Stun, "STUN", "朦朧とさせる"),
    FlagTable::new(Mbe::Hungry, "HUNGRY", "空腹を進行させる"),
    FlagTable::new(Mbe::Chaos, "CHAOS", "カオスを呼び起こす"),
    FlagTable::new(Mbe::Flavor, "FLAVOR", "フレーバー攻撃"),
];

pub trait MonsterRaceFlag: Copy + PartialEq + Eq {
    fn to_flag_str(self) -> &'static str;
    fn description(self) -> &'static str;
}

impl MonsterRace {
    pub fn new() -> Self {
        Self {
            hp: MonsterHitDice { num: 1, sides: 1 },
            vision: 20,

            ..Default::default()
        }
    }

    pub fn to_monster_race_definition(&self) -> String {
        let mut result = String::new();

        result += &format!("N:XXX:{}\n", self.name);
        result += &format!("E:{}\n", self.english_name);
        result += &format!("G:{}:{}\n", self.symbol.char, self.symbol.color.to_char());
        result += &format!(
            "I:{}:{}d{}:{}:{}:{}\n",
            self.speed + 110,
            self.hp.num,
            self.hp.sides,
            self.vision,
            self.ac,
            self.alertness
        );
        result += &format!(
            "W:{}:{}:{}:{}:{}\n",
            self.level, self.rarity, self.exp, self.evolving_exp, self.evolves_to
        );
        for blow in self.blows.iter() {
            result += &make_blow_line(blow);
        }
        result += make_flag_lines("X", [self.sex].as_slice()).as_str();

        result
    }
}

fn make_blow_line(blow: &MonsterBlow) -> String {
    let mut result = String::new();
    if blow.method == MonsterBrowMethod::None {
        return result;
    }

    result += &format!(
        "B:{}:{}",
        blow.method.to_flag_str(),
        blow.effect.to_flag_str(),
    );

    if blow.has_damage {
        result += &format!(":{}d{}", blow.damage_dice.num, blow.damage_dice.sides);
    }

    result.push('\n');

    result
}

fn make_flag_lines<T: MonsterRaceFlag>(header: &str, flags: &[T]) -> String {
    let flag_str_list: Vec<_> = flags.iter().map(|f| f.to_flag_str()).collect();
    let mut lines = vec![String::default()];

    for flag_str in flag_str_list {
        if flag_str.is_empty() {
            continue;
        }

        if lines.last().unwrap().len() + flag_str.len() > 80 {
            lines.push(String::default());
        }

        let current_line = lines.last_mut().unwrap();
        if current_line.is_empty() {
            current_line.push_str(&format!("{header}:{flag_str}"));
        } else {
            current_line.push_str(&format!(" | {flag_str}"));
        }
    }

    let mut result = lines.join("\n");

    if !result.is_empty() {
        result.push('\n');
    }

    result
}

impl MonsterRaceFlag for MonsterSex {
    fn to_flag_str(self) -> &'static str {
        match self {
            MonsterSex::None => "",
            MonsterSex::Male => "MALE",
            MonsterSex::Female => "FEMALE",
        }
    }

    fn description(self) -> &'static str {
        match self {
            MonsterSex::None => "区別なし",
            MonsterSex::Male => "男性/雄",
            MonsterSex::Female => "女性/雌",
        }
    }
}

fn flag_to_str<T: MonsterRaceFlag>(flag: T, tables: &[FlagTable<T>]) -> &'static str {
    tables
        .iter()
        .find(|ft| ft.flag == flag)
        .map(|ft| ft.flag_str)
        .unwrap_or("(undefined)")
}

fn flag_to_description<T: MonsterRaceFlag>(flag: T, tables: &[FlagTable<T>]) -> &'static str {
    tables
        .iter()
        .find(|ft| ft.flag == flag)
        .map(|ft| ft.description)
        .unwrap_or("(undefined)")
}

impl MonsterRaceFlag for MonsterBrowMethod {
    fn to_flag_str(self) -> &'static str {
        flag_to_str(self, &MONSTER_BLOW_METHOD_TABLES)
    }

    fn description(self) -> &'static str {
        flag_to_description(self, &MONSTER_BLOW_METHOD_TABLES)
    }
}

impl MonsterRaceFlag for MonsterBrowEffect {
    fn to_flag_str(self) -> &'static str {
        flag_to_str(self, &MONSTER_BLOW_EFFECT_TABLES)
    }

    fn description(self) -> &'static str {
        flag_to_description(self, &MONSTER_BLOW_EFFECT_TABLES)
    }
}
