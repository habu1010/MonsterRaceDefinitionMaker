use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillBolt {
    Shoot,
    Acid,
    Elec,
    Fire,
    Cold,
    Nether,
    Water,
    Mana,
    Plasma,
    Ice,
    Missile,
    Void,
    Abyss,
    Meteor,
    Lite,
}

use MonsterSkillBolt::*;

pub const MONSTER_SKILL_BOLT_TABLES: [FlagTable<MonsterSkillBolt>; 15] = [
    FlagTable::new(Shoot, "SHOOT", "射撃"),
    FlagTable::new(Acid, "BO_ACID", "アシッド・ボルト"),
    FlagTable::new(Elec, "BO_ELEC", "サンダー・ボルト"),
    FlagTable::new(Fire, "BO_FIRE", "ファイア・ボルト"),
    FlagTable::new(Cold, "BO_COLD", "アイス・ボルト"),
    FlagTable::new(Nether, "BO_NETH", "地獄の矢"),
    FlagTable::new(Water, "BO_WATE", "ウォーター・ボルト"),
    FlagTable::new(Mana, "BO_MANA", "魔力の矢"),
    FlagTable::new(Plasma, "BO_PLAS", "プラズマ・ボルト"),
    FlagTable::new(Ice, "BO_ICEE", "極寒の矢"),
    FlagTable::new(Void, "BO_VOID", "ヴォイド・ボルト"),
    FlagTable::new(Abyss, "BO_ABYSS", "アビス・ボルト"),
    FlagTable::new(Meteor, "BO_METEOR", "メテオストライク"),
    FlagTable::new(Lite, "BO_LITE", "スターライトアロー"),
    FlagTable::new(Missile, "MISSILE", "マジック・ミサイル"),
];

impl MonsterRaceFlag for MonsterSkillBolt {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_SKILL_BOLT_TABLES
    }
}
