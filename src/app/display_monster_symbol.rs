use rand::Rng;

use crate::{color, monster};

const IMAGE_OBJECTS: &[u8; 18] = br#"?/|\"!$()_-=[]{},~"#;
const IMAGE_MONSTERS: &[u8; 52] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Default)]
pub struct DisplayMonsterSymbol {
    symbol: monster::MonsterSymbol,
    prev_update_time: Option<instant::Instant>,

    rng: rand::rngs::ThreadRng,
}

impl DisplayMonsterSymbol {
    pub fn show(&mut self, ui: &mut egui::Ui, monster_race: &monster::MonsterRace) {
        self.determine_symbol(monster_race);

        ui.label(
            egui::RichText::new(&self.symbol.char)
                .monospace()
                .size(24.0)
                .background_color(egui::Color32::BLACK)
                .color(self.symbol.color),
        );
    }

    fn determine_symbol(&mut self, monster_race: &monster::MonsterRace) {
        use monster::MonsterVisual::*;
        let visual = &monster_race.flags.visual;
        let is_multi_color = visual.contains(&MultiColor);
        let is_any_color = visual.contains(&AnyColor);
        let is_shape_changer = visual.contains(&ShapeChanger);

        if !is_multi_color && !is_shape_changer {
            self.symbol = monster_race.symbol.clone();
            return;
        }

        if let Some(prev_update_time) = self.prev_update_time {
            if prev_update_time.elapsed().as_secs_f32() < 1.0 {
                return;
            }
        }
        self.prev_update_time = Some(instant::Instant::now());

        if is_multi_color {
            self.determine_random_symbol_color(is_any_color);
        } else {
            self.symbol.color = monster_race.symbol.color;
        }
        if is_shape_changer {
            self.determine_random_symbol_char();
        } else {
            self.symbol.char = monster_race.symbol.char.clone();
        }
    }

    fn determine_random_symbol_color(&mut self, any_color: bool) {
        let candidates = if any_color {
            &color::COLORS[1..] // Black以外全て
        } else {
            use color::Color::*;
            &[Red, Green, Blue, LightRed, LightGreen, White, DarkGray]
        }
        .iter()
        .filter(|c| self.symbol.color != **c)
        .collect::<Vec<_>>();

        let index = self.rng.gen_range(0..candidates.len());
        self.symbol.color = *candidates[index];
    }

    fn determine_random_symbol_char(&mut self) {
        let candidates = if self.rng.gen_bool(1.0 / 25.0) {
            IMAGE_OBJECTS.as_slice()
        } else {
            IMAGE_MONSTERS.as_slice()
        }
        .iter()
        .filter(|c| c.to_string() != self.symbol.char)
        .collect::<Vec<_>>();

        let index = self.rng.gen_range(0..candidates.len());
        self.symbol.char = String::from(*candidates[index] as char);
    }
}
