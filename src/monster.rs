use std::collections::BTreeSet;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterRace {
    pub name: String,
    pub english_name: String,

    pub symbol: MonsterSymbol,

    pub hp: MonsterHitDice,
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
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterSymbol {
    pub char: String,
    pub color: crate::color::Color,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterHitDice {
    pub num: i32,
    pub sides: i32,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct MonsterBlow {
    pub method: MonsterBrowMethod,
    pub effect: MonsterBrowEffect,
    pub has_damage: bool,
    pub damage_dice: MonsterHitDice,
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

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterSex {
    #[default]
    None,
    Male,
    Female,
}

pub struct FlagTable<T> {
    pub flag: T,
    pub token: &'static str,
    pub description: &'static str,
}

impl<T> FlagTable<T> {
    const fn new(flag: T, token: &'static str, description: &'static str) -> Self {
        Self {
            flag,
            token,
            description,
        }
    }
}

#[derive(
    Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
pub enum MonsterBrowMethod {
    #[default]
    None,
    Hit,
    Touch,
    Punch,
    Kick,
    Claw,
    Bite,
    Sting,
    Slash,
    Butt,
    Crush,
    Engulf,
    Change,
    Crawl,
    Drool,
    Spit,
    Explode,
    Gaze,
    Wail,
    Spore,
    Beg,
    Insult,
    Moan,
    Show,
    Shoot,
}

use MonsterBrowMethod as Mbm;

pub const MONSTER_BLOW_METHOD_TABLES: [FlagTable<MonsterBrowMethod>; 25] = [
    FlagTable::new(Mbm::None, "", "(なし)"),
    FlagTable::new(Mbm::Hit, "HIT", "殴る"),
    FlagTable::new(Mbm::Touch, "TOUCH", "触れる"),
    FlagTable::new(Mbm::Punch, "PUNCH", "パンチする"),
    FlagTable::new(Mbm::Kick, "KICK", "蹴る"),
    FlagTable::new(Mbm::Claw, "CLAW", "ひっかく"),
    FlagTable::new(Mbm::Bite, "BITE", "噛む"),
    FlagTable::new(Mbm::Sting, "STING", "刺す"),
    FlagTable::new(Mbm::Slash, "SLASH", "斬る"),
    FlagTable::new(Mbm::Butt, "BUTT", "角で突く"),
    FlagTable::new(Mbm::Crush, "CRUSH", "体当たりする"),
    FlagTable::new(Mbm::Engulf, "ENGULF", "飲み込む"),
    FlagTable::new(Mbm::Change, "CHANGE", "請求書をよこす"),
    FlagTable::new(Mbm::Crawl, "CRAWL", "体の上を這い回る"),
    FlagTable::new(Mbm::Drool, "DROOL", "よだれを垂らす"),
    FlagTable::new(Mbm::Spit, "SPIT", "つばを吐く"),
    FlagTable::new(Mbm::Explode, "EXPLODE", "爆発する"),
    FlagTable::new(Mbm::Gaze, "GAZE", "にらむ"),
    FlagTable::new(Mbm::Wail, "WAIL", "泣き叫ぶ"),
    FlagTable::new(Mbm::Spore, "SPORE", "胞子を飛ばす"),
    FlagTable::new(Mbm::Beg, "BEG", "金をせがむ"),
    FlagTable::new(Mbm::Insult, "INSULT", "侮辱する"),
    FlagTable::new(Mbm::Moan, "MOAN", "うめく"),
    FlagTable::new(Mbm::Show, "SHOW", "歌う"),
    FlagTable::new(Mbm::Shoot, "SHOOT", "射撃する"),
];

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

use MonsterBrowEffect as Mbe;

pub const MONSTER_BLOW_EFFECT_TABLES: [FlagTable<MonsterBrowEffect>; 39] = [
    FlagTable::new(Mbe::None, "FLAVOR", "(なし)"), // FLAVORと同じ扱いにする
    FlagTable::new(Mbe::Hurt, "HURT", "攻撃する"),
    FlagTable::new(Mbe::SuperHurt, "SUPERHURT", "強力に攻撃する"),
    FlagTable::new(Mbe::Poison, "POISON", "毒をくらわす"),
    FlagTable::new(Mbe::UnBonus, "UN_BONUS", "劣化させる"),
    FlagTable::new(Mbe::UnPower, "UN_POWER", "充填魔力を吸収する"),
    FlagTable::new(Mbe::EatGold, "EAT_GOLD", "金を盗む"),
    FlagTable::new(Mbe::EatItem, "EAT_ITEM", "アイテムを盗む"),
    FlagTable::new(Mbe::EatFood, "EAT_FOOD", "あなたの食料を食べる"),
    FlagTable::new(Mbe::EatLite, "EAT_LITE", "明かりを吸収する"),
    FlagTable::new(Mbe::Acid, "ACID", "酸を飛ばす"),
    FlagTable::new(Mbe::Elec, "ELEC", "感電させる"),
    FlagTable::new(Mbe::Fire, "FIRE", "燃やす"),
    FlagTable::new(Mbe::Cold, "COLD", "凍らせる"),
    FlagTable::new(Mbe::Blind, "BLIND", "盲目にする"),
    FlagTable::new(Mbe::Confuse, "CONFUSE", "混乱させる"),
    FlagTable::new(Mbe::Terrify, "TERRIFY", "恐怖させる"),
    FlagTable::new(Mbe::Paralyze, "PARALYZE", "麻痺させる"),
    FlagTable::new(Mbe::LoseStr, "LOSE_STR", "筋力を減少させる"),
    FlagTable::new(Mbe::LoseInt, "LOSE_INT", "知能を減少させる"),
    FlagTable::new(Mbe::LoseWis, "LOSE_WIS", "賢さを減少させる"),
    FlagTable::new(Mbe::LoseDex, "LOSE_DEX", "器用さを減少させる"),
    FlagTable::new(Mbe::LoseCon, "LOSE_CON", "耐久力を減少させる"),
    FlagTable::new(Mbe::LoseChr, "LOSE_CHR", "魅力を減少させる"),
    FlagTable::new(Mbe::LoseAll, "LOSE_ALL", "全ステータスを減少させる"),
    FlagTable::new(Mbe::Shatter, "SHATTER", "粉砕する"),
    FlagTable::new(Mbe::Exp10, "EXP_10", "経験値を減少(10d6+)させる"),
    FlagTable::new(Mbe::Exp20, "EXP_20", "経験値を減少(20d6+)させる"),
    FlagTable::new(Mbe::Exp40, "EXP_40", "経験値を減少(40d6+)させる"),
    FlagTable::new(Mbe::Exp80, "EXP_80", "経験値を減少(80d6+)させる"),
    FlagTable::new(Mbe::Disease, "DISEASE", "病気にする"),
    FlagTable::new(Mbe::Time, "TIME", "時間を逆戻りさせる"),
    FlagTable::new(Mbe::DrainLife, "EXP_VAMP", "生命力を吸収する"),
    FlagTable::new(Mbe::DrainMana, "DR_MANA", "魔力を奪う"),
    FlagTable::new(Mbe::Inertia, "INERTIA", "減速させる"),
    FlagTable::new(Mbe::Stun, "STUN", "朦朧とさせる"),
    FlagTable::new(Mbe::Hungry, "HUNGRY", "空腹を進行させる"),
    FlagTable::new(Mbe::Chaos, "CHAOS", "カオスを呼び起こす"),
    FlagTable::new(Mbe::Flavor, "FLAVOR", "フレーバー攻撃"),
];

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

use MonsterSkillBreath as Msbr;

pub const MONSTER_SKILL_BREATH_TABLES: [FlagTable<MonsterSkillBreath>; 24] = [
    FlagTable::new(Msbr::Acid, "BR_ACID", "酸のブレス"),
    FlagTable::new(Msbr::Elec, "BR_ELEC", "稲妻のブレス"),
    FlagTable::new(Msbr::Fire, "BR_FIRE", "火炎のブレス"),
    FlagTable::new(Msbr::Cold, "BR_COLD", "冷気のブレス"),
    FlagTable::new(Msbr::Poison, "BR_POIS", "毒のブレス"),
    FlagTable::new(Msbr::Nether, "BR_NETH", "地獄のブレス"),
    FlagTable::new(Msbr::Lite, "BR_LITE", "閃光のブレス"),
    FlagTable::new(Msbr::Dark, "BR_DARK", "暗黒のブレス"),
    FlagTable::new(Msbr::Confusion, "BR_CONF", "混乱のブレス"),
    FlagTable::new(Msbr::Sound, "BR_SOUN", "轟音のブレス"),
    FlagTable::new(Msbr::Chaos, "BR_CHAO", "カオスのブレス"),
    FlagTable::new(Msbr::Disenchant, "BR_DISE", "劣化のブレス"),
    FlagTable::new(Msbr::Nexus, "BR_NEXU", "因果混乱のブレス"),
    FlagTable::new(Msbr::Time, "BR_TIME", "時間逆転のブレス"),
    FlagTable::new(Msbr::Inertia, "BR_INER", "遅鈍のブレス"),
    FlagTable::new(Msbr::Gravity, "BR_GRAV", "重力のブレス"),
    FlagTable::new(Msbr::Shards, "BR_SHAR", "破片のブレス"),
    FlagTable::new(Msbr::Plasma, "BR_PLAS", "プラズマのブレス"),
    FlagTable::new(Msbr::Force, "BR_FORC", "フォースのブレス"),
    FlagTable::new(Msbr::Mana, "BR_MANA", "魔力のブレス"),
    FlagTable::new(Msbr::Nuke, "BR_NUKE", "放射性廃棄物のブレス"),
    FlagTable::new(Msbr::Disintegration, "BR_DISI", "分解のブレス"),
    FlagTable::new(Msbr::Void, "BR_VOID", "虚無のブレス"),
    FlagTable::new(Msbr::Abyss, "BR_ABYSS", "深淵のブレス"),
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillBall {
    Rocket,
    Nuke,
    Chaos,
    Acid,
    Elec,
    Fire,
    Cold,
    Poison,
    Nether,
    Water,
    Mana,
    Dark,
    Lite,
    Void,
    Abyss,
    Meteor,
}

use MonsterSkillBall as Msba;

pub const MONSTER_SKILL_BALL_TABLES: [FlagTable<MonsterSkillBall>; 16] = [
    FlagTable::new(Msba::Rocket, "ROCKET", "ロケット"),
    FlagTable::new(Msba::Acid, "BA_ACID", "アシッド・ボール"),
    FlagTable::new(Msba::Elec, "BA_ELEC", "サンダー・ボール"),
    FlagTable::new(Msba::Fire, "BA_FIRE", "ファイア・ボール"),
    FlagTable::new(Msba::Cold, "BA_COLD", "アイス・ボール"),
    FlagTable::new(Msba::Poison, "BA_POIS", "悪臭雲"),
    FlagTable::new(Msba::Nether, "BA_NETH", "地獄球"),
    FlagTable::new(Msba::Water, "BA_WATE", "ウォーター・ボール"),
    FlagTable::new(Msba::Nuke, "BA_NUKE", "放射能球"),
    FlagTable::new(Msba::Mana, "BA_MANA", "魔力の嵐"),
    FlagTable::new(Msba::Dark, "BA_DARK", "暗黒の嵐"),
    FlagTable::new(Msba::Lite, "BA_LITE", "スターバースト"),
    FlagTable::new(Msba::Chaos, "BA_CHAO", "純ログルス"),
    FlagTable::new(Msba::Void, "BA_VOID", "虚無の嵐"),
    FlagTable::new(Msba::Abyss, "BA_ABYSS", "深淵の嵐"),
    FlagTable::new(Msba::Meteor, "BA_METEOR", "メテオスォーム"),
];

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

use MonsterSkillBolt as Msbo;

pub const MONSTER_SKILL_BOLT_TABLES: [FlagTable<MonsterSkillBolt>; 15] = [
    FlagTable::new(Msbo::Shoot, "SHOOT", "射撃"),
    FlagTable::new(Msbo::Acid, "BO_ACID", "アシッド・ボルト"),
    FlagTable::new(Msbo::Elec, "BO_ELEC", "サンダー・ボルト"),
    FlagTable::new(Msbo::Fire, "BO_FIRE", "ファイア・ボルト"),
    FlagTable::new(Msbo::Cold, "BO_COLD", "アイス・ボルト"),
    FlagTable::new(Msbo::Nether, "BO_NETH", "地獄の矢"),
    FlagTable::new(Msbo::Water, "BO_WATE", "ウォーター・ボルト"),
    FlagTable::new(Msbo::Mana, "BO_MANA", "魔力の矢"),
    FlagTable::new(Msbo::Plasma, "BO_PLAS", "プラズマ・ボルト"),
    FlagTable::new(Msbo::Ice, "BO_ICEE", "極寒の矢"),
    FlagTable::new(Msbo::Void, "BO_VOID", "ヴォイド・ボルト"),
    FlagTable::new(Msbo::Abyss, "BO_ABYSS", "アビス・ボルト"),
    FlagTable::new(Msbo::Meteor, "BO_METEOR", "メテオストライク"),
    FlagTable::new(Msbo::Lite, "BO_LITE", "スターライトアロー"),
    FlagTable::new(Msbo::Missile, "MISSILE", "マジック・ミサイル"),
];

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

use MonsterSkillDamage as Msd;

pub const MONSTER_SKILL_DAMAGE_TABLES: [FlagTable<MonsterSkillDamage>; 9] = [
    FlagTable::new(Msd::HandDoom, "HAND_DOOM", "破滅の手"),
    FlagTable::new(Msd::PsySpear, "PSY_SPEAR", "光の剣"),
    FlagTable::new(Msd::DrainMana, "DRAIN_MANA", "魔力吸収"),
    FlagTable::new(Msd::MindBlast, "MIND_BLAST", "精神攻撃"),
    FlagTable::new(Msd::BrainSmash, "BRAIN_SMASH", "脳攻撃"),
    FlagTable::new(Msd::Cause1, "CAUSE_1", "軽傷+呪い"),
    FlagTable::new(Msd::Cause2, "CAUSE_2", "重傷+呪い"),
    FlagTable::new(Msd::Cause3, "CAUSE_3", "致命傷+呪い"),
    FlagTable::new(Msd::Cause4, "CAUSE_4", "秘孔を突く"),
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillSummon {
    Monster,
    Monsters,
    Kin,
    Ant,
    Spider,
    Hound,
    Hydra,
    Angel,
    Demon,
    Undead,
    Dragon,
    HiUndead,
    HiDragon,
    Cyber,
    Amberites,
    Unique,
    DeadUnique,
}

use MonsterSkillSummon as Mss;

pub const MONSTER_SKILL_SUMMON_TABLES: [FlagTable<MonsterSkillSummon>; 17] = [
    FlagTable::new(Mss::Monster, "S_MONSTER", "モンスター一体召喚"),
    FlagTable::new(Mss::Monsters, "S_MONSTERS", "モンスター複数召喚"),
    FlagTable::new(Mss::Kin, "S_KIN", "救援召喚"),
    FlagTable::new(Mss::Ant, "S_ANT", "アリ召喚"),
    FlagTable::new(Mss::Spider, "S_SPIDER", "クモ召喚"),
    FlagTable::new(Mss::Hound, "S_HOUND", "ハウンド召喚"),
    FlagTable::new(Mss::Hydra, "S_HYDRA", "ヒドラ召喚"),
    FlagTable::new(Mss::Angel, "S_ANGEL", "天使一体召喚"),
    FlagTable::new(Mss::Demon, "S_DEMON", "デーモン一体召喚"),
    FlagTable::new(Mss::Undead, "S_UNDEAD", "アンデッド一体召喚"),
    FlagTable::new(Mss::Dragon, "S_DRAGON", "ドラゴン一体召喚"),
    FlagTable::new(Mss::HiUndead, "S_HI_UNDEAD", "強力なアンデッド召喚"),
    FlagTable::new(Mss::HiDragon, "S_HI_DRAGON", "古代ドラゴン召喚"),
    FlagTable::new(Mss::Cyber, "S_CYBER", "サイバーデーモン召喚"),
    FlagTable::new(Mss::Amberites, "S_AMBERITES", "アンバーの王族召喚"),
    FlagTable::new(Mss::Unique, "S_UNIQUE", "ユニーク・モンスター召喚"),
    FlagTable::new(
        Mss::DeadUnique,
        "S_DEAD_UNIQUE",
        "ユニーク・モンスター口寄せ",
    ),
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum MonsterSkillMisc {
    Shriek,
    Scare,
    Blind,
    Confusion,
    Slow,
    Hold,
    Haste,
    Heal,
    Invulnerability,
    Dispel,
    Blink,
    Teleport,
    TeleportTo,
    TeleportAway,
    TeleportLevel,
    World,
    Darkness,
    Traps,
    Forget,
    RaiseDead,
    Special,
}

use MonsterSkillMisc as Msm;

pub const MONSTER_SKILL_MISC_TABLES: [FlagTable<MonsterSkillMisc>; 21] = [
    FlagTable::new(Msm::Shriek, "SHRIEK", "悲鳴を上げる"),
    FlagTable::new(Msm::Scare, "SCARE", "恐怖"),
    FlagTable::new(Msm::Blind, "BLIND", "目くらまし"),
    FlagTable::new(Msm::Confusion, "CONF", "混乱"),
    FlagTable::new(Msm::Slow, "SLOW", "減速"),
    FlagTable::new(Msm::Hold, "HOLD", "麻痺"),
    FlagTable::new(Msm::Haste, "HASTE", "加速"),
    FlagTable::new(Msm::Heal, "HEAL", "治癒"),
    FlagTable::new(Msm::Invulnerability, "INVULNER", "無敵化"),
    FlagTable::new(Msm::Dispel, "DISPEL", "魔法消去"),
    FlagTable::new(Msm::Blink, "BLINK", "ショートテレポート"),
    FlagTable::new(Msm::Teleport, "TPORT", "テレポート"),
    FlagTable::new(Msm::TeleportTo, "TELE_TO", "テレポートバック"),
    FlagTable::new(Msm::TeleportAway, "TELE_AWAY", "テレポートアウェイ"),
    FlagTable::new(Msm::TeleportLevel, "TELE_LEVEL", "テレポート・レベル"),
    FlagTable::new(Msm::World, "WORLD", "時を止める"),
    FlagTable::new(Msm::Darkness, "DARKNESS", "暗闇"),
    FlagTable::new(Msm::Traps, "TRAPS", "トラップ"),
    FlagTable::new(Msm::Forget, "FORGET", "記憶消去"),
    FlagTable::new(Msm::RaiseDead, "ANIM_DEAD", "死者復活"),
    FlagTable::new(Msm::Special, "SPECIAL", "特別な行動をする"),
];

pub trait MonsterRaceFlag {
    fn token(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

impl MonsterRace {
    pub fn new() -> Self {
        Self {
            hp: MonsterHitDice { num: 1, sides: 1 },
            vision: 20,
            skill_use_prob_div: 1,

            ..Default::default()
        }
    }

    pub fn to_monster_race_definition(&self) -> String {
        let mut result = String::new();

        result += &format!("N:XXX:{}\n", self.name);
        result += &format!("E:{}\n", self.english_name);
        result += &format!("G:{}:{}\n", self.symbol.char, self.symbol.color.to_char());
        result += &format!(
            "I:{}:{}d{}:{}:{}:{}\n",
            self.speed + 110,
            self.hp.num,
            self.hp.sides,
            self.vision,
            self.ac,
            self.alertness
        );
        result += &format!(
            "W:{}:{}:{}:{}:{}\n",
            self.level, self.rarity, self.exp, self.evolving_exp, self.evolves_to
        );
        for blow in self.blows.iter() {
            result += &make_blow_line(blow);
        }
        result += make_flag_lines("X", [self.sex.token()].as_slice()).as_str();

        let mut skill_flags = Vec::new();
        skill_flags.extend(self.skill.breathes.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.balls.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.bolts.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.damages.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.summons.iter().map(|f| f.token()));
        skill_flags.extend(self.skill.miscs.iter().map(|f| f.token()));
        if !skill_flags.is_empty() {
            result += &format!("S:1_IN_{}\n", self.skill_use_prob_div);
            result += make_flag_lines("S", &skill_flags).as_str();
        }

        result
    }
}

fn make_blow_line(blow: &MonsterBlow) -> String {
    let mut result = String::new();
    if blow.method == MonsterBrowMethod::None {
        return result;
    }

    result += &format!("B:{}:{}", blow.method.token(), blow.effect.token(),);

    if blow.has_damage {
        result += &format!(":{}d{}", blow.damage_dice.num, blow.damage_dice.sides);
    }

    result.push('\n');

    result
}

fn make_flag_lines(header: &str, token_list: &[&str]) -> String {
    let mut lines = vec![String::default()];

    for &token in token_list {
        if token.is_empty() {
            continue;
        }

        if lines.last().unwrap().len() + token.len() > 80 {
            lines.push(String::default());
        }

        let current_line = lines.last_mut().unwrap();
        if current_line.is_empty() {
            current_line.push_str(&format!("{header}:{token}"));
        } else {
            current_line.push_str(&format!(" | {token}"));
        }
    }

    let mut result = lines.join("\n");

    if !result.is_empty() {
        result.push('\n');
    }

    result
}

impl MonsterRaceFlag for MonsterSex {
    fn token(&self) -> &'static str {
        match self {
            MonsterSex::None => "",
            MonsterSex::Male => "MALE",
            MonsterSex::Female => "FEMALE",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            MonsterSex::None => "区別なし",
            MonsterSex::Male => "男性/雄",
            MonsterSex::Female => "女性/雌",
        }
    }
}

fn flag_to_token<T: MonsterRaceFlag + Eq>(flag: &T, tables: &[FlagTable<T>]) -> &'static str {
    tables
        .iter()
        .find(|ft| ft.flag == *flag)
        .map(|ft| ft.token)
        .unwrap_or("(undefined)")
}

fn flag_to_description<T: MonsterRaceFlag + Eq>(flag: &T, tables: &[FlagTable<T>]) -> &'static str {
    tables
        .iter()
        .find(|ft| ft.flag == *flag)
        .map(|ft| ft.description)
        .unwrap_or("(undefined)")
}

impl MonsterRaceFlag for MonsterBrowMethod {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_BLOW_METHOD_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_BLOW_METHOD_TABLES)
    }
}

impl MonsterRaceFlag for MonsterBrowEffect {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_BLOW_EFFECT_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_BLOW_EFFECT_TABLES)
    }
}

impl MonsterRaceFlag for MonsterSkillBreath {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_SKILL_BREATH_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_SKILL_BREATH_TABLES)
    }
}

impl MonsterRaceFlag for MonsterSkillBall {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_SKILL_BALL_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_SKILL_BALL_TABLES)
    }
}

impl MonsterRaceFlag for MonsterSkillBolt {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_SKILL_BOLT_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_SKILL_BOLT_TABLES)
    }
}

impl MonsterRaceFlag for MonsterSkillDamage {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_SKILL_DAMAGE_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_SKILL_DAMAGE_TABLES)
    }
}

impl MonsterRaceFlag for MonsterSkillSummon {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_SKILL_SUMMON_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_SKILL_SUMMON_TABLES)
    }
}

impl MonsterRaceFlag for MonsterSkillMisc {
    fn token(&self) -> &'static str {
        flag_to_token(self, &MONSTER_SKILL_MISC_TABLES)
    }

    fn description(&self) -> &'static str {
        flag_to_description(self, &MONSTER_SKILL_MISC_TABLES)
    }
}
