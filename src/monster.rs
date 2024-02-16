mod flags;

use std::{collections::BTreeSet, fmt::Write};

use crate::util::HitDice;
pub use flags::*;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterRace {
    pub name: String,
    pub english_name: String,

    pub symbol: MonsterSymbol,

    pub hp: HitDice,
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

    pub skill: MonsterSkill,
    pub skill_use_prob_div: i32,

    pub flags: MonsterFlags,

    pub escort_num: usize,
    pub escorts: [MonsterEscort; 6],

    pub flavor: String,
    pub english_flavor: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterSymbol {
    pub char: String,
    pub color: crate::color::Color,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterBlow {
    pub method: MonsterBrowMethod,
    pub effect: MonsterBrowEffect,
    pub has_damage: bool,
    pub damage_dice: HitDice,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterSkill {
    pub breathes: BTreeSet<MonsterSkillBreath>,
    pub balls: BTreeSet<MonsterSkillBall>,
    pub bolts: BTreeSet<MonsterSkillBolt>,
    pub damages: BTreeSet<MonsterSkillDamage>,
    pub summons: BTreeSet<MonsterSkillSummon>,
    pub miscs: BTreeSet<MonsterSkillMisc>,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterFlags {
    pub aura: BTreeSet<MonsterAura>,
    pub behavior: BTreeSet<MonsterBehavior>,
    pub brightness: BTreeSet<MonsterBrightness>,
    pub drop: BTreeSet<MonsterDrop>,
    pub feature: BTreeSet<MonsterFeature>,
    pub kind: BTreeSet<MonsterKind>,
    pub misc: BTreeSet<MonsterMisc>,
    pub population: BTreeSet<MonsterPopulation>,
    pub resistance: BTreeSet<MonsterResistance>,
    pub speak: BTreeSet<MonsterSpeak>,
    pub visual: BTreeSet<MonsterVisual>,
    pub weakness: BTreeSet<MonsterWeakness>,
    pub wildness: BTreeSet<MonsterWildness>,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterEscort {
    pub monster_id: u32,
    pub num: HitDice,
}

impl MonsterFlags {
    fn collect_enabled_tokens(&self) -> Vec<&str> {
        let mut result = Vec::new();

        result.extend(self.kind.iter().map(|f| f.token()));
        result.extend(self.drop.iter().map(|f| f.token()));
        result.extend(self.visual.iter().map(|f| f.token()));
        result.extend(self.behavior.iter().map(|f| f.token()));
        result.extend(self.feature.iter().map(|f| f.token()));
        result.extend(self.brightness.iter().map(|f| f.token()));
        result.extend(self.misc.iter().map(|f| f.token()));
        result.extend(self.population.iter().map(|f| f.token()));
        result.extend(self.speak.iter().map(|f| f.token()));
        result.extend(self.aura.iter().map(|f| f.token()));
        result.extend(self.resistance.iter().map(|f| f.token()));
        result.extend(self.weakness.iter().map(|f| f.token()));
        result.extend(self.wildness.iter().map(|f| f.token()));

        result
    }
}

impl MonsterRace {
    pub fn new() -> Self {
        Self {
            vision: 20,
            skill_use_prob_div: 1,

            ..Default::default()
        }
    }

    pub fn to_monster_race_definition(&self) -> String {
        let mut result = String::new();

        writeln!(result, "N:XXX:{}", self.name).unwrap();
        writeln!(result, "E:{}", self.english_name).unwrap();
        writeln!(
            result,
            "G:{}:{}",
            self.symbol.char,
            self.symbol.color.to_char()
        )
        .unwrap();
        writeln!(
            result,
            "I:{}:{}d{}:{}:{}:{}",
            self.speed + 110,
            self.hp.num,
            self.hp.sides,
            self.vision,
            self.ac,
            self.alertness
        )
        .unwrap();
        writeln!(
            result,
            "W:{}:{}:{}:{}:{}",
            self.level, self.rarity, self.exp, self.evolving_exp, self.evolves_to
        )
        .unwrap();
        for blow in self.blows.iter() {
            result += &make_blow_line(blow);
        }
        result += make_flag_lines("X", [self.sex.token()].as_slice()).as_str();

        result += make_flag_lines("F", &self.flags.collect_enabled_tokens()).as_str();

        let mut skill_flags = Vec::new();
        skill_flags.extend(self.skill.breathes.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.balls.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.bolts.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.damages.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.summons.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.miscs.iter().map(|f| f.token()));
        if !skill_flags.is_empty() {
            writeln!(result, "S:1_IN_{}", self.skill_use_prob_div).unwrap();
            result += make_flag_lines("S", &skill_flags).as_str();
        }

        result += make_escorts_lines(self.escort_num, &self.escorts).as_str();

        result += make_flavor_lines("D:$", &self.english_flavor).as_str();
        result += make_flavor_lines("D:", &self.flavor).as_str();

        result
    }
}

fn make_blow_line(blow: &MonsterBlow) -> String {
    let mut result = String::new();
    if blow.method == MonsterBrowMethod::None {
        return result;
    }

    write!(result, "B:{}:{}", blow.method.token(), blow.effect.token()).unwrap();

    if blow.has_damage {
        let HitDice { num, sides } = blow.damage_dice;
        write!(result, ":{num}d{sides}").unwrap();
    }

    result.push('\n');

    result
}

fn make_flag_lines(header: &str, token_list: &[&str]) -> String {
    let mut lines = vec![String::default()];

    for &token in token_list {
        if token.is_empty() {
            continue;
        }

        if lines.last().unwrap().len() + token.len() > 80 {
            lines.push(String::default());
        }

        let current_line = lines.last_mut().unwrap();
        if current_line.is_empty() {
            write!(current_line, "{header}:{token}").unwrap();
        } else {
            write!(current_line, " | {token}").unwrap();
        }
    }

    let mut result = lines.join("\n");

    if !result.is_empty() {
        result.push('\n');
    }

    result
}

fn make_flavor_lines(header: &str, flavor: &str) -> String {
    let mut lines = String::new();

    for line in flavor
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        writeln!(lines, "{header}{line}").unwrap();
    }

    lines
}

fn make_escorts_lines(escort_num: usize, escorts: &[MonsterEscort]) -> String {
    let mut result = String::new();

    for escort in escorts.iter().take(escort_num) {
        writeln!(
            result,
            "R:{}:{}d{}\n",
            escort.monster_id, escort.num.num, escort.num.sides
        )
        .unwrap();
    }

    result
}
