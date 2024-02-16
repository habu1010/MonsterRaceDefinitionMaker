use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use crate::{
    color,
    monster::{self, MonsterRaceFlag},
    search,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MonsterRaceDefinitionMakerApp {
    selected_side_panel_item: SidePanelItem,

    monster_race: monster::MonsterRace,

    #[serde(skip)]
    search_ctx: SearchCtx,
}

struct SearchCtx {
    db: search::MonsterRaceDataBase,
    results: Vec<Rc<search::MonsterRace>>,
    query: String,
    monster_id: u32,
}

impl Default for MonsterRaceDefinitionMakerApp {
    fn default() -> Self {
        Self {
            monster_race: monster::MonsterRace::new(),
            selected_side_panel_item: SidePanelItem::MonsterRaceBasicInfo,
            search_ctx: SearchCtx::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum SidePanelItem {
    MonsterRaceBasicInfo,
    MonsterRaceBlows,
    MonsterRaceSkills1,
    MonsterRaceSkills2,
    MonsterRaceFlags1,
    MonsterRaceFlags2,
    MonsterRaceEscorts,
    MonsterRaceArtifactDrop,
    MonsterRaceFlavor,
    MonsterSearch,
    Export,
}

impl MonsterRaceDefinitionMakerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            String::from("Noto Sans JP"),
            egui::FontData::from_static(include_bytes!("../assets/NotoSansJP-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, String::from("Noto Sans JP"));
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push(String::from("Noto Sans JP"));

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn draw_symbol_field(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            egui::Grid::new("symbol field")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("シンボル:");
                    egui::TextEdit::singleline(&mut self.monster_race.symbol.char)
                        .desired_width(24.0)
                        .char_limit(1)
                        .show(ui);
                    ui.end_row();
                    ui.label("色:");
                    egui::ComboBox::from_id_source("symbol color")
                        .selected_text(format!("{}", self.monster_race.symbol.color))
                        .show_ui(ui, |ui| {
                            for color in color::COLORS {
                                ui.selectable_value(
                                    &mut self.monster_race.symbol.color,
                                    color,
                                    format!("{}", color),
                                );
                            }
                        });
                    ui.end_row();
                    ui.label("見た目:");
                    ui.label(
                        egui::RichText::new(&self.monster_race.symbol.char)
                            .monospace()
                            .size(24.0)
                            .background_color(egui::Color32::BLACK)
                            .color(self.monster_race.symbol.color.to_color32()),
                    );
                });
        });
    }

    fn draw_info_field(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            egui::Grid::new("I field")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("HP:");
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.monster_race.hp.num)
                                .clamp_range(1..=1000),
                        );
                        ui.label("d");
                        ui.add(
                            egui::DragValue::new(&mut self.monster_race.hp.sides)
                                .clamp_range(1..=1000),
                        );
                    });
                    ui.end_row();

                    let force_max_hp = self
                        .monster_race
                        .flags
                        .misc
                        .contains(&monster::MonsterMisc::ForceMaxHp);
                    if force_max_hp {
                        ui.label(" 最大値(固定)");
                        ui.label(format!("{}", self.monster_race.hp.max()));
                    } else {
                        ui.label(" 期待値");
                        ui.label(format!("{:.1}", self.monster_race.hp.average()));
                    }
                    ui.end_row();

                    let mut drag_value_form = |label, value, range| {
                        ui.label(label);
                        ui.add(egui::DragValue::new(value).clamp_range(range));
                        ui.end_row();
                    };
                    drag_value_form("加速:", &mut self.monster_race.speed, -100..=100);
                    drag_value_form("AC:", &mut self.monster_race.ac, 0..=10000);
                    drag_value_form("感知範囲:", &mut self.monster_race.vision, 1..=255);
                    drag_value_form("警戒度:", &mut self.monster_race.alertness, 0..=255);
                });
        });
    }

    fn draw_more_info_field(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            egui::Grid::new("W field")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    let mut drag_value_form = |label, value, range| {
                        ui.label(label);
                        ui.add(egui::DragValue::new(value).clamp_range(range));
                        ui.end_row();
                    };

                    drag_value_form("階層:", &mut self.monster_race.level, 0..=127);
                    drag_value_form("レア度:", &mut self.monster_race.rarity, 1..=255);
                    drag_value_form("経験値:", &mut self.monster_race.exp, 0..=1000000);
                    drag_value_form(
                        "進化経験値:",
                        &mut self.monster_race.evolving_exp,
                        0..=1000000,
                    );
                    drag_value_form("進化先:", &mut self.monster_race.evolves_to, 0..=9999);
                });
        });
    }

    fn draw_sex_info_field(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            use monster::MonsterSex::*;
            ui.vertical(|ui| {
                ui.label("性別:");
                for sex in [None, Male, Female] {
                    ui.radio_value(&mut self.monster_race.sex, sex, sex.description());
                }
            });
        });
    }

    fn update_basic_info(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("name field")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("日本語名:");
                egui::TextEdit::singleline(&mut self.monster_race.name)
                    .desired_width(f32::INFINITY)
                    .show(ui);
                ui.end_row();
                ui.label("英語名:");
                egui::TextEdit::singleline(&mut self.monster_race.english_name)
                    .desired_width(f32::INFINITY)
                    .show(ui);
                ui.end_row();
            });

        ui.horizontal_top(|ui| {
            self.draw_symbol_field(ui);
            self.draw_info_field(ui);
            self.draw_more_info_field(ui);
            self.draw_sex_info_field(ui);
        });
    }

    fn update_blows_info(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("blows field")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for (i, blow) in self.monster_race.blows.iter_mut().enumerate() {
                    ui.label(&format!("攻撃{}:", i + 1));
                    ui.horizontal_top(|ui| {
                        combo_box_from_frag_tables(
                            ui,
                            &format!("blow method {i}"),
                            &mut blow.method,
                            &monster::MONSTER_BLOW_METHOD_TABLES,
                        );
                        combo_box_from_frag_tables(
                            ui,
                            &format!("blow effect {i}"),
                            &mut blow.effect,
                            &monster::MONSTER_BLOW_EFFECT_TABLES,
                        );
                    });
                    ui.end_row();
                    ui.label("");
                    ui.horizontal_top(|ui| {
                        ui.checkbox(&mut blow.has_damage, "ダメージ");
                        if blow.has_damage {
                            ui.add(
                                egui::DragValue::new(&mut blow.damage_dice.num)
                                    .clamp_range(1..=1000),
                            );
                            ui.label("d");
                            ui.add(
                                egui::DragValue::new(&mut blow.damage_dice.sides)
                                    .clamp_range(1..=1000),
                            );
                        }
                    });
                    ui.end_row();
                }
            });
    }

    fn update_skills_info1(&mut self, ui: &mut egui::Ui) {
        self.update_skill_use_prob(ui);
        ui.horizontal(|ui| {
            check_box_list_from_flag_tables(
                ui,
                "ブレス",
                &mut self.monster_race.skill.breathes,
                &monster::MONSTER_SKILL_BREATH_TABLES,
            );
            check_box_list_from_flag_tables(
                ui,
                "ボール",
                &mut self.monster_race.skill.balls,
                &monster::MONSTER_SKILL_BALL_TABLES,
            );
            check_box_list_from_flag_tables(
                ui,
                "ボルト",
                &mut self.monster_race.skill.bolts,
                &monster::MONSTER_SKILL_BOLT_TABLES,
            );
        });
    }

    fn update_skills_info2(&mut self, ui: &mut egui::Ui) {
        self.update_skill_use_prob(ui);
        ui.horizontal(|ui| {
            check_box_list_from_flag_tables(
                ui,
                "ダメージ",
                &mut self.monster_race.skill.damages,
                &monster::MONSTER_SKILL_DAMAGE_TABLES,
            );
            check_box_list_from_flag_tables(
                ui,
                "召喚",
                &mut self.monster_race.skill.summons,
                &monster::MONSTER_SKILL_SUMMON_TABLES,
            );
            check_box_list_from_flag_tables(
                ui,
                "その他",
                &mut self.monster_race.skill.miscs,
                &monster::MONSTER_SKILL_MISC_TABLES,
            );
        });
    }

    fn update_skill_use_prob(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("使用率: 1_IN_");
            ui.add(
                egui::DragValue::new(&mut self.monster_race.skill_use_prob_div)
                    .clamp_range(1..=100),
            );
        });
    }

    fn update_flags_info1(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            check_box_list_from_flag_tables(
                ui,
                "耐性",
                &mut self.monster_race.flags.resistance,
                &monster::MONSTER_RESISTANCE_TABLES,
            );
            ui.vertical(|ui| {
                check_box_list_from_flag_tables(
                    ui,
                    "オーラ",
                    &mut self.monster_race.flags.aura,
                    &monster::MONSTER_AURA_TABLES,
                );
                check_box_list_from_flag_tables(
                    ui,
                    "弱点",
                    &mut self.monster_race.flags.weakness,
                    &monster::MONSTER_WEAKNESS_TABLES,
                );
                check_box_list_from_flag_tables(
                    ui,
                    "ドロップ",
                    &mut self.monster_race.flags.drop,
                    &monster::MONSTER_DROP_TABLES,
                );
            });
            ui.vertical(|ui| {
                check_box_list_from_flag_tables(
                    ui,
                    "種族/属性",
                    &mut self.monster_race.flags.kind,
                    &monster::MONSTER_KIND_TABLES,
                );
                check_box_list_from_flag_tables(
                    ui,
                    "外見",
                    &mut self.monster_race.flags.visual,
                    &monster::MONSTER_VISUAL_TABLES,
                );
            });

            ui.vertical(|ui| {
                check_box_list_from_flag_tables(
                    ui,
                    "行動",
                    &mut self.monster_race.flags.behavior,
                    &monster::MONSTER_BEHAVIOR_TABLES,
                );
                check_box_list_from_flag_tables(
                    ui,
                    "特性",
                    &mut self.monster_race.flags.feature,
                    &monster::MONSTER_FEATURE_TABLES,
                );
            });
        });
    }

    fn update_flags_info2(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                check_box_list_from_flag_tables(
                    ui,
                    "光源",
                    &mut self.monster_race.flags.brightness,
                    &monster::MONSTER_BRIGHTNESS_TABLES,
                );
                check_box_list_from_flag_tables(
                    ui,
                    "生成数",
                    &mut self.monster_race.flags.population,
                    &monster::MONSTER_POPULATION_TABLES,
                );
                check_box_list_from_flag_tables(
                    ui,
                    "会話",
                    &mut self.monster_race.flags.speak,
                    &monster::MONSTER_SPEAK_TABLES,
                );
            });
            check_box_list_from_flag_tables(
                ui,
                "地上生成",
                &mut self.monster_race.flags.wildness,
                &monster::MONSTER_WILDNESS_TABLES,
            );
            check_box_list_from_flag_tables(
                ui,
                "その他",
                &mut self.monster_race.flags.misc,
                &monster::MONSTER_MISC_TABLES,
            );
        });
    }

    fn update_escorts(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("護衛種別数:");
            ui.add(
                egui::DragValue::new(&mut self.monster_race.escort_num)
                    .clamp_range(0..=self.monster_race.escorts.len()),
            );
        });
        for escort in self
            .monster_race
            .escorts
            .iter_mut()
            .take(self.monster_race.escort_num)
        {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("モンスターID:");
                    ui.add(
                        egui::DragValue::new(&mut escort.monster_id)
                            .clamp_range(0..=self.search_ctx.db.id_range().max().unwrap()),
                    );
                    let name = match self.search_ctx.db.get(escort.monster_id) {
                        Ok(m) => m.name.to_string(),
                        Err(_) => "不明".to_string(),
                    };

                    ui.text_edit_singleline(&mut name.as_str());
                });
                ui.horizontal(|ui| {
                    ui.label("数:");
                    ui.add(egui::DragValue::new(&mut escort.num.num).clamp_range(1..=9));
                    ui.label("d");
                    ui.add(egui::DragValue::new(&mut escort.num.sides).clamp_range(1..=9));
                });
            });
        }
    }

    fn update_artifact_drops(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("ドロップ種別数:");
            ui.add(
                egui::DragValue::new(&mut self.monster_race.drop_artifact_num)
                    .clamp_range(0..=self.monster_race.drop_artifacts.len()),
            );
        });
        if self.monster_race.drop_artifact_num == 0 {
            return;
        }

        ui.group(|ui| {
            egui::Grid::new("drop artifact field")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("固定アーティファクトID");
                    ui.label("ドロップ確率(%)");
                    ui.end_row();
                    for drop_artifact in self
                        .monster_race
                        .drop_artifacts
                        .iter_mut()
                        .take(self.monster_race.drop_artifact_num)
                    {
                        ui.add(
                            egui::DragValue::new(&mut drop_artifact.artifact_id)
                                .clamp_range(0..=9999),
                        );
                        ui.add(
                            egui::DragValue::new(&mut drop_artifact.prob_percent)
                                .clamp_range(0..=100),
                        );
                        ui.end_row();
                    }
                });
        });

        ui.hyperlink_to(
            "固定アーティファクト定義ファイル参照",
            "https://raw.githubusercontent.com/hengband/hengband/develop/lib/edit/ArtifactDefinitions.txt"
        );
    }

    fn update_flavor(&mut self, ui: &mut egui::Ui) {
        ui.label("日本語フレーバー:");
        egui::ScrollArea::vertical()
            .id_source("japanese flavor")
            .max_height(200.0)
            .auto_shrink(false)
            .show(ui, |ui| {
                egui::TextEdit::multiline(&mut self.monster_race.flavor)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });
        ui.add_space(10.0);
        ui.label("英語フレーバー:");
        egui::ScrollArea::vertical()
            .id_source("english flavor")
            .max_height(200.0)
            .auto_shrink(false)
            .show(ui, |ui| {
                egui::TextEdit::multiline(&mut self.monster_race.english_flavor)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });
    }

    fn update_export(&mut self, ui: &mut egui::Ui) {
        let mut monster_race_definition = self.monster_race.to_monster_race_definition();

        if ui.button("クリップボードにコピー").clicked() {
            ui.output_mut(|o| o.copied_text = monster_race_definition.clone());
        }

        egui::TextEdit::multiline(&mut monster_race_definition)
            .desired_width(f32::INFINITY)
            .show(ui);
    }
}

impl eframe::App for MonsterRaceDefinitionMakerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        use SidePanelItem::*;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // The side panel is often a good place for tools and settings.
            let side_panel = &mut self.selected_side_panel_item;
            ui.selectable_value(side_panel, MonsterRaceBasicInfo, "基本情報");
            ui.selectable_value(side_panel, MonsterRaceBlows, "近接攻撃");
            ui.selectable_value(side_panel, MonsterRaceSkills1, "スキル1");
            ui.selectable_value(side_panel, MonsterRaceSkills2, "スキル2");
            ui.selectable_value(side_panel, MonsterRaceFlags1, "フラグ1");
            ui.selectable_value(side_panel, MonsterRaceFlags2, "フラグ2");
            ui.selectable_value(side_panel, MonsterRaceEscorts, "護衛");
            ui.selectable_value(side_panel, MonsterRaceArtifactDrop, "固定AFドロップ");
            ui.selectable_value(side_panel, MonsterRaceFlavor, "フレーバーテキスト");
            ui.selectable_value(side_panel, MonsterSearch, "モンスター検索");
            ui.selectable_value(side_panel, Export, "エクスポート");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(env!("CARGO_PKG_VERSION"));
                ui.label("Monster Race Definition Maker");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.selected_side_panel_item {
            MonsterRaceBasicInfo => self.update_basic_info(ui),
            MonsterRaceBlows => self.update_blows_info(ui),
            MonsterRaceSkills1 => self.update_skills_info1(ui),
            MonsterRaceSkills2 => self.update_skills_info2(ui),
            MonsterRaceFlags1 => self.update_flags_info1(ui),
            MonsterRaceFlags2 => self.update_flags_info2(ui),
            MonsterRaceEscorts => self.update_escorts(ui),
            MonsterRaceArtifactDrop => self.update_artifact_drops(ui),
            MonsterRaceFlavor => self.update_flavor(ui),
            MonsterSearch => self.search_ctx.update(ui),
            Export => self.update_export(ui),
        });
    }
}

impl SearchCtx {
    fn new() -> Self {
        Self {
            db: search::MonsterRaceDataBase::new(),
            results: Vec::new(),
            query: String::new(),
            monster_id: 0,
        }
    }

    fn update(&mut self, ui: &mut egui::Ui) {
        let Self {
            db,
            results,
            query,
            monster_id,
        } = self;

        ui.heading("モンスター検索");
        ui.horizontal(|ui| {
            if ui
                .add(
                    egui::TextEdit::singleline(query)
                        .desired_width(200.0)
                        .hint_text("モンスター名の一部を入力"),
                )
                .changed()
            {
                *results = if query.is_empty() {
                    Vec::default()
                } else {
                    db.search(query)
                };
            }
            ui.label(&format!("検索結果: {}件", results.len()));
        });

        egui::ScrollArea::vertical()
            .max_height(150.0)
            .auto_shrink(false)
            .show(ui, |ui| {
                egui::Grid::new("search result grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("ID");
                        ui.label("名前");
                        ui.end_row();
                        for monster in results {
                            if ui
                                .add_sized(
                                    ui.available_size(),
                                    egui::Button::new(&monster.id.to_string()),
                                )
                                .clicked()
                            {
                                *monster_id = monster.id;
                            }
                            ui.label(&monster.name);
                            ui.end_row();
                        }
                    });
            });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("ID入力:");
            ui.add(egui::DragValue::new(monster_id).clamp_range(db.id_range()));
        });

        let monster = db.get(*monster_id);

        let mut definition = match monster {
            Ok(ref monster) => monster.definition.as_str().trim_end(),
            Err(search::SearchError::Preparing) => "データの準備中です",
            Err(search::SearchError::IdNotFound) => "該当のモンスターIDが見つかりません",
            Err(search::SearchError::FailedToDownload) => "データのダウンロードに失敗しました",
        };
        ui.group(|ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    egui::TextEdit::multiline(&mut definition)
                        .desired_width(f32::INFINITY)
                        .show(ui);
                });
        });
    }
}

fn combo_box_from_frag_tables<T>(
    ui: &mut egui::Ui,
    id_source: &str,
    selected: &mut T,
    tables: &[monster::FlagTable<T>],
) where
    T: MonsterRaceFlag + Copy + Eq,
{
    egui::ComboBox::from_id_source(id_source)
        .selected_text(selected.description())
        .show_ui(ui, |ui| {
            for t in tables {
                ui.selectable_value(selected, t.flag, t.description);
            }
        });
}

fn check_box_list_from_flag_tables<T>(
    ui: &mut egui::Ui,
    header: &str,
    flags: &mut BTreeSet<T>,
    tables: &[monster::FlagTable<T>],
) where
    T: MonsterRaceFlag + Copy + Ord,
{
    let mut flag_map = tables
        .iter()
        .map(|t| (t.flag, flags.contains(&t.flag)))
        .collect::<BTreeMap<_, _>>();
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.heading(header);
            for (flag, selected) in flag_map.iter_mut() {
                ui.checkbox(selected, flag.description())
                    .on_hover_text(flag.token());
            }
        });
    });
    *flags = flag_map
        .iter()
        .filter_map(
            |(flag, selected)| {
                if *selected {
                    Some(*flag)
                } else {
                    None
                }
            },
        )
        .collect();
}
