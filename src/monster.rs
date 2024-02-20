mod flags;

use std::{collections::BTreeSet, fmt::Write};

use crate::{color, util::HitDice};
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

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct MonsterSymbol {
    pub char: String,
    pub color: color::Color,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterBlow {
    pub method: MonsterBlowMethod,
    pub effect: MonsterBlowEffect,
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

    fn enable_from_token(&mut self, token: &str) -> Result<(), String> {
        use MonsterSkillGroup::*;
        match token.parse::<MonsterSkillGroup>()? {
            Breath(skill) => self.breathes.insert(skill),
            Ball(skill) => self.balls.insert(skill),
            Bolt(skill) => self.bolts.insert(skill),
            Damage(skill) => self.damages.insert(skill),
            Summon(skill) => self.summons.insert(skill),
            Misc(skill) => self.miscs.insert(skill),
        };

        Ok(())
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

    fn enable_from_token(&mut self, token: &str) -> Result<(), String> {
        use MonsterFlagGroup::*;
        match token.parse::<MonsterFlagGroup>()? {
            Kind(flag) => self.kind.insert(flag),
            Drop(flag) => self.drop.insert(flag),
            Visual(flag) => self.visual.insert(flag),
            Behavior(flag) => self.behavior.insert(flag),
            Feature(flag) => self.feature.insert(flag),
            Brightness(flag) => self.brightness.insert(flag),
            Misc(flag) => self.misc.insert(flag),
            Population(flag) => self.population.insert(flag),
            Speak(flag) => self.speak.insert(flag),
            Aura(flag) => self.aura.insert(flag),
            Resistance(flag) => self.resistance.insert(flag),
            Weakness(flag) => self.weakness.insert(flag),
            Wildness(flag) => self.wildness.insert(flag),
        };

        Ok(())
    }
}

impl std::str::FromStr for MonsterRace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monster_race = Self::new();
        let mut line_num = 0;

        for line in s.lines().map(str::trim_end) {
            line_num += 1;
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Err(err) = monster_race.parse_line(line) {
                return Err(format!("Error at line {line_num}: {err}"));
            }
        }

        Ok(monster_race)
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

    fn parse_line(&mut self, line: &str) -> Result<(), String> {
        let parts = line.split(':').collect::<Vec<_>>();
        if parts.len() < 2 {
            return Err(format!("Invalid line: {}", line));
        }

        match parts[0] {
            "N" => self.name = parts.get(2).ok_or("Invalid N: field")?.to_string(),
            "E" => self.english_name = parts[1].to_string(),
            "G" => self.parse_symbol_field(&parts)?,
            "I" => self.parse_info_field(&parts)?,
            "W" => self.parse_more_info_field(&parts)?,
            "B" => self.parse_blow_field(&parts)?,
            "X" => self.parse_sex_field(&parts)?,
            "F" => self.parse_flag_field(&parts)?,
            "S" => self.parse_skill_field(&parts)?,
            "R" => self.parse_escort_field(&parts)?,
            "A" => self.parse_artifact_drop_field(&parts)?,
            "V" => self.parse_odds_correction_ratio_field(&parts)?,
            "D" => self.parse_flavor_field(&line[2..])?,
            _ => return Err(format!("Unknown field: {}", parts[0])),
        };

        Ok(())
    }

    pub fn parse_symbol_field(&mut self, cols: &[&str]) -> Result<(), String> {
        if cols.len() != 3 || cols[2].chars().count() != 1 {
            return Err("Invalid G field format".to_string());
        }

        let color_symbol = color::ColorSymbol(cols[2].chars().next().unwrap());
        self.symbol.char = cols[1].to_string();
        self.symbol.color = color::Color::try_from(color_symbol)?;

        Ok(())
    }

    pub fn parse_info_field(&mut self, cols: &[&str]) -> Result<(), String> {
        let err_msg = || "Invalid I field format".to_string();
        if cols.len() != 6 {
            return Err(err_msg());
        }

        self.speed = cols[1].parse().map_err(|_| err_msg())?;
        self.speed -= 110;
        self.hp = cols[2].parse()?;
        self.vision = cols[3].parse().map_err(|_| err_msg())?;
        self.ac = cols[4].parse().map_err(|_| err_msg())?;
        self.alertness = cols[5].parse().map_err(|_| err_msg())?;

        Ok(())
    }

    pub fn parse_more_info_field(&mut self, cols: &[&str]) -> Result<(), String> {
        let err_msg = || "Invalid W field format".to_string();
        if cols.len() != 6 {
            return Err(err_msg());
        }

        self.level = cols[1].parse().map_err(|_| err_msg())?;
        self.rarity = cols[2].parse().map_err(|_| err_msg())?;
        self.exp = cols[3].parse().map_err(|_| err_msg())?;
        self.evolving_exp = cols[4].parse().map_err(|_| err_msg())?;
        self.evolves_to = cols[5].parse().map_err(|_| err_msg())?;

        Ok(())
    }

    pub fn parse_blow_field(&mut self, cols: &[&str]) -> Result<(), String> {
        if cols.len() != 3 && cols.len() != 4 {
            return Err("Invalid B field format".to_string());
        }

        let blow = MonsterBlow {
            method: MonsterBlowMethod::from_token(cols[1])?,
            effect: MonsterBlowEffect::from_token(cols[2])?,
            has_damage: cols.len() > 3,
            damage_dice: match cols.get(3) {
                Some(dice) => dice.parse()?,
                None => HitDice::default(),
            },
        };

        for i in 0..4 {
            if self.blows[i].method == MonsterBlowMethod::None {
                self.blows[i] = blow;
                return Ok(());
            }
        }

        Err("Too many B fields".to_string())
    }

    pub fn parse_sex_field(&mut self, cols: &[&str]) -> Result<(), String> {
        if cols.len() != 2 {
            return Err("Invalid X field format".to_string());
        }

        self.sex = MonsterSex::from_token(cols[1])?;

        Ok(())
    }

    pub fn parse_flag_field(&mut self, cols: &[&str]) -> Result<(), String> {
        if cols.len() != 2 {
            return Err("Invalid F field format".to_string());
        }

        for token in cols[1].split('|').map(str::trim).filter(|t| !t.is_empty()) {
            if self.flags.enable_from_token(token).is_ok() {
                continue;
            }

            return Err(format!("Unknown F token: {}", token));
        }

        Ok(())
    }

    pub fn parse_skill_field(&mut self, cols: &[&str]) -> Result<(), String> {
        if cols.len() != 2 {
            return Err("Invalid S field format".to_string());
        }

        for token in cols[1].split('|').map(str::trim).filter(|t| !t.is_empty()) {
            if self.skill.enable_from_token(token).is_ok() {
                continue;
            }
            if let Some(div) = token.strip_prefix("1_IN_") {
                if let Ok(div) = div.parse() {
                    self.skill_use_prob_div = div;
                    continue;
                }
            }

            return Err(format!("Unknown S token: {}", token));
        }

        Ok(())
    }

    pub fn parse_escort_field(&mut self, cols: &[&str]) -> Result<(), String> {
        let err_msg = || "Invalid R field format".to_string();
        if cols.len() != 3 {
            return Err(err_msg());
        }

        if self.escort_num >= self.escorts.len() {
            return Err("Too many R fields".to_string());
        }

        let monster_id = cols[1].parse().map_err(|_| err_msg())?;
        let num = cols[2].parse()?;

        self.escorts[self.escort_num] = MonsterEscort { monster_id, num };
        self.escort_num += 1;

        Ok(())
    }

    pub fn parse_artifact_drop_field(&mut self, cols: &[&str]) -> Result<(), String> {
        let err_msg = || "Invalid A field format".to_string();
        if cols.len() != 3 {
            return Err(err_msg());
        }

        if self.drop_artifact_num >= self.drop_artifacts.len() {
            return Err("Too many A field".to_string());
        }

        let artifact_id = cols[1].parse().map_err(|_| err_msg())?;
        let prob_percent = cols[2].parse().map_err(|_| err_msg())?;

        self.drop_artifacts[self.drop_artifact_num] = MonsterArtifactDrop {
            artifact_id,
            prob_percent,
        };
        self.drop_artifact_num += 1;

        Ok(())
    }

    pub fn parse_odds_correction_ratio_field(&mut self, cols: &[&str]) -> Result<(), String> {
        let err_msg = || "Invalid V field format".to_string();
        if cols.len() != 2 {
            return Err(err_msg());
        }

        self.odds_correction_ratio = cols[1].parse().map_err(|_| err_msg())?;

        Ok(())
    }

    pub fn parse_flavor_field(&mut self, line: &str) -> Result<(), String> {
        let line = line.trim();

        if let Some(stripped) = line.strip_prefix('$') {
            self.english_flavor += stripped;
            self.english_flavor.push('\n');
        } else {
            self.flavor += line;
            self.flavor.push('\n');
        }

        Ok(())
    }

    pub fn to_monster_race_definition(&self) -> String {
        let mut result = String::new();

        writeln!(result, "N:XXX:{}", self.name).unwrap();
        writeln!(result, "E:{}", self.english_name).unwrap();
        writeln!(
            result,
            "G:{}:{}",
            self.symbol.char,
            color::ColorSymbol::from(self.symbol.color)
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
    if blow.method == MonsterBlowMethod::None {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monster_definition() {
        let file = include_str!("../MonsterRaceDefinitions.txt");

        let mut block = String::new();
        for line in file.lines().skip_while(|line| !line.starts_with("N:")) {
            if line.starts_with("N:") {
                if let Err(e) = block.parse::<MonsterRace>() {
                    panic!("Error parsing block: {}\n{}", e, block);
                }
                block.clear();
            }

            block += line;
            block.push('\n');
        }
    }
}
