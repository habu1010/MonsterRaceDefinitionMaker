use crate::{
    color,
    monster::{self, MonsterRaceFlag},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MonsterRaceDefinitionMakerApp {
    selected_side_panel_item: SidePanelItem,

    monster_race: monster::MonsterRace,
}

impl Default for MonsterRaceDefinitionMakerApp {
    fn default() -> Self {
        Self {
            monster_race: monster::MonsterRace::new(),
            selected_side_panel_item: SidePanelItem::MonsterRaceBasicInfo,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum SidePanelItem {
    MonsterRaceBasicInfo,
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

    fn update_basic_info(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
        });
    }

    fn update_export(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut monster_race_definition = self.monster_race.to_monster_race_definition();

            if ui.button("クリップボードにコピー").clicked() {
                ui.output_mut(|o| o.copied_text = monster_race_definition.clone());
            }

            egui::TextEdit::multiline(&mut monster_race_definition).show(ui);
        });
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
            ui.selectable_value(side_panel, Export, "エクスポート");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });

        match self.selected_side_panel_item {
            MonsterRaceBasicInfo => self.update_basic_info(ctx),
            Export => self.update_export(ctx),
        }
    }
}
