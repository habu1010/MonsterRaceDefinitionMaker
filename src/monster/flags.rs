mod aura;
mod behavior;
mod blow_effect;
mod blow_method;
mod brightness;
mod drop;
mod feature;
mod kind;
mod misc;
mod population;
mod resistance;
mod sex;
mod skill_ball;
mod skill_bolt;
mod skill_breath;
mod skill_damage;
mod skill_misc;
mod skill_summon;
mod speak;
mod visual;
mod weakness;
mod wildness;

pub use aura::*;
pub use behavior::*;
pub use blow_effect::*;
pub use blow_method::*;
pub use brightness::*;
pub use drop::*;
pub use feature::*;
pub use kind::*;
pub use misc::*;
pub use population::*;
pub use resistance::*;
pub use sex::*;
pub use skill_ball::*;
pub use skill_bolt::*;
pub use skill_breath::*;
pub use skill_damage::*;
pub use skill_misc::*;
pub use skill_summon::*;
pub use speak::*;
pub use visual::*;
pub use weakness::*;
pub use wildness::*;

pub struct FlagTableRow<T> {
    pub flag: T,
    pub token: &'static str,
    pub description: &'static str,
}

impl<T> FlagTableRow<T> {
    const fn new(flag: T, token: &'static str, description: &'static str) -> Self {
        Self {
            flag,
            token,
            description,
        }
    }
}

pub trait MonsterRaceFlag: Sized + Copy + Eq + 'static {
    fn get_flag_table() -> &'static [FlagTableRow<Self>];

    fn token(&self) -> &'static str {
        Self::get_flag_table()
            .iter()
            .find(|ft| ft.flag == *self)
            .map(|ft| ft.token)
            .unwrap_or("(undefined token)")
    }

    fn description(&self) -> &'static str {
        Self::get_flag_table()
            .iter()
            .find(|ft| ft.flag == *self)
            .map(|ft| ft.description)
            .unwrap_or("(undefined description)")
    }

    fn from_token(token: &str) -> Result<Self, String> {
        Self::get_flag_table()
            .iter()
            .find(|ft| ft.token == token)
            .map(|ft| ft.flag)
            .ok_or_else(|| format!("Unknown token: {}", token))
    }
}

pub enum MonsterSkillGroup {
    Breath(MonsterSkillBreath),
    Ball(MonsterSkillBall),
    Bolt(MonsterSkillBolt),
    Damage(MonsterSkillDamage),
    Misc(MonsterSkillMisc),
    Summon(MonsterSkillSummon),
}

impl std::str::FromStr for MonsterSkillGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(breath) = MonsterSkillBreath::from_token(s) {
            return Ok(Self::Breath(breath));
        }
        if let Ok(ball) = MonsterSkillBall::from_token(s) {
            return Ok(Self::Ball(ball));
        }
        if let Ok(bolt) = MonsterSkillBolt::from_token(s) {
            return Ok(Self::Bolt(bolt));
        }
        if let Ok(damage) = MonsterSkillDamage::from_token(s) {
            return Ok(Self::Damage(damage));
        }
        if let Ok(misc) = MonsterSkillMisc::from_token(s) {
            return Ok(Self::Misc(misc));
        }
        if let Ok(summon) = MonsterSkillSummon::from_token(s) {
            return Ok(Self::Summon(summon));
        }

        Err(format!("Unknown skill token: {}", s))
    }
}

pub enum MonsterFlagGroup {
    Aura(MonsterAura),
    Behavior(MonsterBehavior),
    Brightness(MonsterBrightness),
    Drop(MonsterDrop),
    Feature(MonsterFeature),
    Kind(MonsterKind),
    Misc(MonsterMisc),
    Population(MonsterPopulation),
    Resistance(MonsterResistance),
    Speak(MonsterSpeak),
    Visual(MonsterVisual),
    Weakness(MonsterWeakness),
    Wildness(MonsterWildness),
}

impl std::str::FromStr for MonsterFlagGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(aura) = MonsterAura::from_token(s) {
            return Ok(Self::Aura(aura));
        }
        if let Ok(behavior) = MonsterBehavior::from_token(s) {
            return Ok(Self::Behavior(behavior));
        }
        if let Ok(brightness) = MonsterBrightness::from_token(s) {
            return Ok(Self::Brightness(brightness));
        }
        if let Ok(drop) = MonsterDrop::from_token(s) {
            return Ok(Self::Drop(drop));
        }
        if let Ok(feature) = MonsterFeature::from_token(s) {
            return Ok(Self::Feature(feature));
        }
        if let Ok(kind) = MonsterKind::from_token(s) {
            return Ok(Self::Kind(kind));
        }
        if let Ok(misc) = MonsterMisc::from_token(s) {
            return Ok(Self::Misc(misc));
        }
        if let Ok(population) = MonsterPopulation::from_token(s) {
            return Ok(Self::Population(population));
        }
        if let Ok(resistance) = MonsterResistance::from_token(s) {
            return Ok(Self::Resistance(resistance));
        }
        if let Ok(speak) = MonsterSpeak::from_token(s) {
            return Ok(Self::Speak(speak));
        }
        if let Ok(visual) = MonsterVisual::from_token(s) {
            return Ok(Self::Visual(visual));
        }
        if let Ok(weakness) = MonsterWeakness::from_token(s) {
            return Ok(Self::Weakness(weakness));
        }
        if let Ok(wildness) = MonsterWildness::from_token(s) {
            return Ok(Self::Wildness(wildness));
        }

        Err(format!("Unknown flag token: {}", s))
    }
}
