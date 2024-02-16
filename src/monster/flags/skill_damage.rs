use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillDamage {
    HandDoom,
    PsySpear,
    DrainMana,
    MindBlast,
    BrainSmash,
    Cause1,
    Cause2,
    Cause3,
    Cause4,
}

use MonsterSkillDamage::*;

const MONSTER_SKILL_DAMAGE_TABLE: [FlagTableRow<MonsterSkillDamage>; 9] = [
    FlagTableRow::new(HandDoom, "HAND_DOOM", "破滅の手"),
    FlagTableRow::new(PsySpear, "PSY_SPEAR", "光の剣"),
    FlagTableRow::new(DrainMana, "DRAIN_MANA", "魔力吸収"),
    FlagTableRow::new(MindBlast, "MIND_BLAST", "精神攻撃"),
    FlagTableRow::new(BrainSmash, "BRAIN_SMASH", "脳攻撃"),
    FlagTableRow::new(Cause1, "CAUSE_1", "軽傷+呪い"),
    FlagTableRow::new(Cause2, "CAUSE_2", "重傷+呪い"),
    FlagTableRow::new(Cause3, "CAUSE_3", "致命傷+呪い"),
    FlagTableRow::new(Cause4, "CAUSE_4", "秘孔を突く"),
];

impl MonsterRaceFlag for MonsterSkillDamage {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SKILL_DAMAGE_TABLE
    }
}
