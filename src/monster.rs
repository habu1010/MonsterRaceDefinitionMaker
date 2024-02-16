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

    pub drop_artifact_num: usize,
    pub drop_artifacts: [MonsterArtifactDrop; 6],

    pub odds_correction_ratio: u32,

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

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterArtifactDrop {
    pub artifact_id: u32,
    pub prob_percent: u32,
}

impl MonsterSkill {
    fn collect_enabled_tokens(&self) -> Vec<&str> {
        let mut result = Vec::new();

        result.extend(self.breathes.iter().map(|f| f.token()));
        result.extend(self.balls.iter().map(|f| f.token()));
        result.extend(self.bolts.iter().map(|f| f.token()));
        result.extend(self.damages.iter().map(|f| f.token()));
        result.extend(self.summons.iter().map(|f| f.token()));
        result.extend(self.miscs.iter().map(|f| f.token()));

        result
    }
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
            odds_correction_ratio: 100,

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
            "I:{}:{}:{}:{}:{}",
            self.speed + 110,
            self.hp,
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
            write_blow_lines(&mut result, blow);
        }
        write_flag_lines(&mut result, "X", [self.sex.token()].as_slice());

        write_flag_lines(&mut result, "F", &self.flags.collect_enabled_tokens());

        let skill_tokens = self.skill.collect_enabled_tokens();
        if !skill_tokens.is_empty() {
            writeln!(result, "S:1_IN_{}", self.skill_use_prob_div).unwrap();
            write_flag_lines(&mut result, "S", &skill_tokens);
        }

        for escort in self.escorts.iter().take(self.escort_num) {
            writeln!(result, "R:{}:{}", escort.monster_id, escort.num).unwrap();
        }

        for drop in self.drop_artifacts.iter().take(self.drop_artifact_num) {
            writeln!(result, "A:{}:{}", drop.artifact_id, drop.prob_percent).unwrap();
        }

        if self.odds_correction_ratio != 100 {
            writeln!(result, "V:{}", self.odds_correction_ratio).unwrap();
        }

        write_flavor_lines(&mut result, "D:$", &self.english_flavor);
        write_flavor_lines(&mut result, "D:", &self.flavor);

        result
    }
}

fn write_blow_lines<W: Write>(writer: &mut W, blow: &MonsterBlow) {
    if blow.method == MonsterBrowMethod::None {
        return;
    }

    write!(writer, "B:{}:{}", blow.method.token(), blow.effect.token()).unwrap();

    if blow.has_damage {
        write!(writer, ":{}", blow.damage_dice).unwrap();
    }

    writeln!(writer).unwrap();
}

fn write_flag_lines<W: Write>(writer: &mut W, header: &str, token_list: &[&str]) {
    let mut line_len = 0;

    for &token in token_list.iter().filter(|t| !t.is_empty()) {
        if line_len + token.len() > 80 {
            writeln!(writer).unwrap();
            line_len = 0;
        }

        if line_len == 0 {
            write!(writer, "{header}:{token}").unwrap();
            line_len += header.len() + 1 + token.len();
        } else {
            write!(writer, " | {token}").unwrap();
            line_len += 3 + token.len();
        }
    }

    if line_len > 0 {
        writeln!(writer).unwrap();
    }
}

fn write_flavor_lines<W: Write>(writer: &mut W, header: &str, flavor: &str) {
    for line in flavor
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        writeln!(writer, "{header}{line}").unwrap();
    }
}
