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

pub const MONSTER_SKILL_BREATH_TABLES: [FlagTable<MonsterSkillBreath>; 24] = [
    FlagTable::new(Acid, "BR_ACID", "酸のブレス"),
    FlagTable::new(Elec, "BR_ELEC", "稲妻のブレス"),
    FlagTable::new(Fire, "BR_FIRE", "火炎のブレス"),
    FlagTable::new(Cold, "BR_COLD", "冷気のブレス"),
    FlagTable::new(Poison, "BR_POIS", "毒のブレス"),
    FlagTable::new(Nether, "BR_NETH", "地獄のブレス"),
    FlagTable::new(Lite, "BR_LITE", "閃光のブレス"),
    FlagTable::new(Dark, "BR_DARK", "暗黒のブレス"),
    FlagTable::new(Confusion, "BR_CONF", "混乱のブレス"),
    FlagTable::new(Sound, "BR_SOUN", "轟音のブレス"),
    FlagTable::new(Chaos, "BR_CHAO", "カオスのブレス"),
    FlagTable::new(Disenchant, "BR_DISE", "劣化のブレス"),
    FlagTable::new(Nexus, "BR_NEXU", "因果混乱のブレス"),
    FlagTable::new(Time, "BR_TIME", "時間逆転のブレス"),
    FlagTable::new(Inertia, "BR_INER", "遅鈍のブレス"),
    FlagTable::new(Gravity, "BR_GRAV", "重力のブレス"),
    FlagTable::new(Shards, "BR_SHAR", "破片のブレス"),
    FlagTable::new(Plasma, "BR_PLAS", "プラズマのブレス"),
    FlagTable::new(Force, "BR_FORC", "フォースのブレス"),
    FlagTable::new(Mana, "BR_MANA", "魔力のブレス"),
    FlagTable::new(Nuke, "BR_NUKE", "放射性廃棄物のブレス"),
    FlagTable::new(Disintegration, "BR_DISI", "分解のブレス"),
    FlagTable::new(Void, "BR_VOID", "虚無のブレス"),
    FlagTable::new(Abyss, "BR_ABYSS", "深淵のブレス"),
];

impl MonsterRaceFlag for MonsterSkillBreath {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_SKILL_BREATH_TABLES
    }
}
