use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillBreath {
    Acid,
    Elec,
    Fire,
    Cold,
    Poison,
    Nether,
    Lite,
    Dark,
    Confusion,
    Sound,
    Chaos,
    Disenchant,
    Nexus,
    Time,
    Inertia,
    Gravity,
    Shards,
    Plasma,
    Force,
    Mana,
    Nuke,
    Disintegration,
    Void,
    Abyss,
}

use MonsterSkillBreath::*;

const MONSTER_SKILL_BREATH_TABLE: [FlagTableRow<MonsterSkillBreath>; 24] = [
    FlagTableRow::new(Acid, "BR_ACID", "酸のブレス"),
    FlagTableRow::new(Elec, "BR_ELEC", "稲妻のブレス"),
    FlagTableRow::new(Fire, "BR_FIRE", "火炎のブレス"),
    FlagTableRow::new(Cold, "BR_COLD", "冷気のブレス"),
    FlagTableRow::new(Poison, "BR_POIS", "毒のブレス"),
    FlagTableRow::new(Nether, "BR_NETH", "地獄のブレス"),
    FlagTableRow::new(Lite, "BR_LITE", "閃光のブレス"),
    FlagTableRow::new(Dark, "BR_DARK", "暗黒のブレス"),
    FlagTableRow::new(Confusion, "BR_CONF", "混乱のブレス"),
    FlagTableRow::new(Sound, "BR_SOUN", "轟音のブレス"),
    FlagTableRow::new(Chaos, "BR_CHAO", "カオスのブレス"),
    FlagTableRow::new(Disenchant, "BR_DISE", "劣化のブレス"),
    FlagTableRow::new(Nexus, "BR_NEXU", "因果混乱のブレス"),
    FlagTableRow::new(Time, "BR_TIME", "時間逆転のブレス"),
    FlagTableRow::new(Inertia, "BR_INER", "遅鈍のブレス"),
    FlagTableRow::new(Gravity, "BR_GRAV", "重力のブレス"),
    FlagTableRow::new(Shards, "BR_SHAR", "破片のブレス"),
    FlagTableRow::new(Plasma, "BR_PLAS", "プラズマのブレス"),
    FlagTableRow::new(Force, "BR_FORC", "フォースのブレス"),
    FlagTableRow::new(Mana, "BR_MANA", "魔力のブレス"),
    FlagTableRow::new(Nuke, "BR_NUKE", "放射性廃棄物のブレス"),
    FlagTableRow::new(Disintegration, "BR_DISI", "分解のブレス"),
    FlagTableRow::new(Void, "BR_VOID", "虚無のブレス"),
    FlagTableRow::new(Abyss, "BR_ABYSS", "深淵のブレス"),
];

impl MonsterRaceFlag for MonsterSkillBreath {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_SKILL_BREATH_TABLE
    }
}
