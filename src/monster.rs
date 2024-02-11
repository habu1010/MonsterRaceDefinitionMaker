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

#[derive(Default, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum MonsterSex {
    #[default]
    None,
    Male,
    Female,
}

pub trait MonsterRaceFlag: Copy {
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
        result += make_flag_lines("X", [self.sex].as_slice()).as_str();

        result
    }
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
