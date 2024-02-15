use super::*;

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterBrowEffect {
    #[default]
    None,
    Hurt,
    SuperHurt,
    Poison,
    UnBonus,
    UnPower,
    EatGold,
    EatItem,
    EatFood,
    EatLite,
    Acid,
    Elec,
    Fire,
    Cold,
    Blind,
    Confuse,
    Terrify,
    Paralyze,
    LoseStr,
    LoseInt,
    LoseWis,
    LoseDex,
    LoseCon,
    LoseChr,
    LoseAll,
    Shatter,
    Exp10,
    Exp20,
    Exp40,
    Exp80,
    Disease,
    Time,
    DrainLife,
    DrainMana,
    Inertia,
    Stun,
    Hungry,
    Chaos,
    Flavor,
}

use MonsterBrowEffect::*;

pub const MONSTER_BLOW_EFFECT_TABLES: [FlagTable<MonsterBrowEffect>; 39] = [
    FlagTable::new(None, "FLAVOR", "(なし)"), // FLAVORと同じ扱いにする
    FlagTable::new(Hurt, "HURT", "攻撃する"),
    FlagTable::new(SuperHurt, "SUPERHURT", "強力に攻撃する"),
    FlagTable::new(Poison, "POISON", "毒をくらわす"),
    FlagTable::new(UnBonus, "UN_BONUS", "劣化させる"),
    FlagTable::new(UnPower, "UN_POWER", "充填魔力を吸収する"),
    FlagTable::new(EatGold, "EAT_GOLD", "金を盗む"),
    FlagTable::new(EatItem, "EAT_ITEM", "アイテムを盗む"),
    FlagTable::new(EatFood, "EAT_FOOD", "あなたの食料を食べる"),
    FlagTable::new(EatLite, "EAT_LITE", "明かりを吸収する"),
    FlagTable::new(Acid, "ACID", "酸を飛ばす"),
    FlagTable::new(Elec, "ELEC", "感電させる"),
    FlagTable::new(Fire, "FIRE", "燃やす"),
    FlagTable::new(Cold, "COLD", "凍らせる"),
    FlagTable::new(Blind, "BLIND", "盲目にする"),
    FlagTable::new(Confuse, "CONFUSE", "混乱させる"),
    FlagTable::new(Terrify, "TERRIFY", "恐怖させる"),
    FlagTable::new(Paralyze, "PARALYZE", "麻痺させる"),
    FlagTable::new(LoseStr, "LOSE_STR", "筋力を減少させる"),
    FlagTable::new(LoseInt, "LOSE_INT", "知能を減少させる"),
    FlagTable::new(LoseWis, "LOSE_WIS", "賢さを減少させる"),
    FlagTable::new(LoseDex, "LOSE_DEX", "器用さを減少させる"),
    FlagTable::new(LoseCon, "LOSE_CON", "耐久力を減少させる"),
    FlagTable::new(LoseChr, "LOSE_CHR", "魅力を減少させる"),
    FlagTable::new(LoseAll, "LOSE_ALL", "全ステータスを減少させる"),
    FlagTable::new(Shatter, "SHATTER", "粉砕する"),
    FlagTable::new(Exp10, "EXP_10", "経験値を減少(10d6+)させる"),
    FlagTable::new(Exp20, "EXP_20", "経験値を減少(20d6+)させる"),
    FlagTable::new(Exp40, "EXP_40", "経験値を減少(40d6+)させる"),
    FlagTable::new(Exp80, "EXP_80", "経験値を減少(80d6+)させる"),
    FlagTable::new(Disease, "DISEASE", "病気にする"),
    FlagTable::new(Time, "TIME", "時間を逆戻りさせる"),
    FlagTable::new(DrainLife, "EXP_VAMP", "生命力を吸収する"),
    FlagTable::new(DrainMana, "DR_MANA", "魔力を奪う"),
    FlagTable::new(Inertia, "INERTIA", "減速させる"),
    FlagTable::new(Stun, "STUN", "朦朧とさせる"),
    FlagTable::new(Hungry, "HUNGRY", "空腹を進行させる"),
    FlagTable::new(Chaos, "CHAOS", "カオスを呼び起こす"),
    FlagTable::new(Flavor, "FLAVOR", "フレーバー攻撃"),
];

impl MonsterRaceFlag for MonsterBrowEffect {
    fn get_flag_tables(&self) -> &[FlagTable<Self>] {
        &MONSTER_BLOW_EFFECT_TABLES
    }
}
