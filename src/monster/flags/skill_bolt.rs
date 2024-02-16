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

const MONSTER_SKILL_BOLT_TABLE: [FlagTableRow<MonsterSkillBolt>; 15] = [
    FlagTableRow::new(Shoot, "SHOOT", "射撃"),
    FlagTableRow::new(Acid, "BO_ACID", "アシッド・ボルト"),
    FlagTableRow::new(Elec, "BO_ELEC", "サンダー・ボルト"),
    FlagTableRow::new(Fire, "BO_FIRE", "ファイア・ボルト"),
    FlagTableRow::new(Cold, "BO_COLD", "アイス・ボルト"),
    FlagTableRow::new(Nether, "BO_NETH", "地獄の矢"),
    FlagTableRow::new(Water, "BO_WATE", "ウォーター・ボルト"),
    FlagTableRow::new(Mana, "BO_MANA", "魔力の矢"),
    FlagTableRow::new(Plasma, "BO_PLAS", "プラズマ・ボルト"),
    FlagTableRow::new(Ice, "BO_ICEE", "極寒の矢"),
    FlagTableRow::new(Void, "BO_VOID", "ヴォイド・ボルト"),
    FlagTableRow::new(Abyss, "BO_ABYSS", "アビス・ボルト"),
    FlagTableRow::new(Meteor, "BO_METEOR", "メテオストライク"),
    FlagTableRow::new(Lite, "BO_LITE", "スターライトアロー"),
    FlagTableRow::new(Missile, "MISSILE", "マジック・ミサイル"),
];

impl MonsterRaceFlag for MonsterSkillBolt {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SKILL_BOLT_TABLE
    }
}
