use super::*;

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterBlowEffect {
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

use MonsterBlowEffect::*;

const MONSTER_BLOW_EFFECT_TABLE: [FlagTableRow<MonsterBlowEffect>; 39] = [
    FlagTableRow::new(None, "FLAVOR", "(なし)"), // FLAVORと同じ扱いにする
    FlagTableRow::new(Hurt, "HURT", "攻撃する"),
    FlagTableRow::new(SuperHurt, "SUPERHURT", "強力に攻撃する"),
    FlagTableRow::new(Poison, "POISON", "毒をくらわす"),
    FlagTableRow::new(UnBonus, "UN_BONUS", "劣化させる"),
    FlagTableRow::new(UnPower, "UN_POWER", "充填魔力を吸収する"),
    FlagTableRow::new(EatGold, "EAT_GOLD", "金を盗む"),
    FlagTableRow::new(EatItem, "EAT_ITEM", "アイテムを盗む"),
    FlagTableRow::new(EatFood, "EAT_FOOD", "あなたの食料を食べる"),
    FlagTableRow::new(EatLite, "EAT_LITE", "明かりを吸収する"),
    FlagTableRow::new(Acid, "ACID", "酸を飛ばす"),
    FlagTableRow::new(Elec, "ELEC", "感電させる"),
    FlagTableRow::new(Fire, "FIRE", "燃やす"),
    FlagTableRow::new(Cold, "COLD", "凍らせる"),
    FlagTableRow::new(Blind, "BLIND", "盲目にする"),
    FlagTableRow::new(Confuse, "CONFUSE", "混乱させる"),
    FlagTableRow::new(Terrify, "TERRIFY", "恐怖させる"),
    FlagTableRow::new(Paralyze, "PARALYZE", "麻痺させる"),
    FlagTableRow::new(LoseStr, "LOSE_STR", "筋力を減少させる"),
    FlagTableRow::new(LoseInt, "LOSE_INT", "知能を減少させる"),
    FlagTableRow::new(LoseWis, "LOSE_WIS", "賢さを減少させる"),
    FlagTableRow::new(LoseDex, "LOSE_DEX", "器用さを減少させる"),
    FlagTableRow::new(LoseCon, "LOSE_CON", "耐久力を減少させる"),
    FlagTableRow::new(LoseChr, "LOSE_CHR", "魅力を減少させる"),
    FlagTableRow::new(LoseAll, "LOSE_ALL", "全ステータスを減少させる"),
    FlagTableRow::new(Shatter, "SHATTER", "粉砕する"),
    FlagTableRow::new(Exp10, "EXP_10", "経験値を減少(10d6+)させる"),
    FlagTableRow::new(Exp20, "EXP_20", "経験値を減少(20d6+)させる"),
    FlagTableRow::new(Exp40, "EXP_40", "経験値を減少(40d6+)させる"),
    FlagTableRow::new(Exp80, "EXP_80", "経験値を減少(80d6+)させる"),
    FlagTableRow::new(Disease, "DISEASE", "病気にする"),
    FlagTableRow::new(Time, "TIME", "時間を逆戻りさせる"),
    FlagTableRow::new(DrainLife, "EXP_VAMP", "生命力を吸収する"),
    FlagTableRow::new(DrainMana, "DR_MANA", "魔力を奪う"),
    FlagTableRow::new(Inertia, "INERTIA", "減速させる"),
    FlagTableRow::new(Stun, "STUN", "朦朧とさせる"),
    FlagTableRow::new(Hungry, "HUNGRY", "空腹を進行させる"),
    FlagTableRow::new(Chaos, "CHAOS", "カオスを呼び起こす"),
    FlagTableRow::new(Flavor, "FLAVOR", "フレーバー攻撃"),
];

impl MonsterRaceFlag for MonsterBlowEffect {
    fn get_flag_table() -> &'static [FlagTableRow<Self>] {
        &MONSTER_BLOW_EFFECT_TABLE
    }
}
