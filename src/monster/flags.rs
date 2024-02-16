mod aura;
mod behavior;
mod brightness;
mod brow_effect;
mod brow_method;
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
pub use brightness::*;
pub use brow_effect::*;
pub use brow_method::*;
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

pub trait MonsterRaceFlag: Sized + Eq + 'static {
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
}
