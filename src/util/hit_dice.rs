#[derive(serde::Deserialize, serde::Serialize)]
pub struct HitDice {
    pub num: i32,
    pub sides: i32,
}

impl Default for HitDice {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl HitDice {
    pub const fn new(num: i32, sides: i32) -> Self {
        Self { num, sides }
    }

    pub const fn max(&self) -> i32 {
        self.num * self.sides
    }

    pub fn average(&self) -> f64 {
        self.num as f64 * (self.sides as f64 + 1.0) / 2.0
    }
}

impl std::fmt::Display for HitDice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.num, self.sides)
    }
}
