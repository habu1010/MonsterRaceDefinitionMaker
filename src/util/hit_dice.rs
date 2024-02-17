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

impl std::str::FromStr for HitDice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = || format!("Invalid hit dice: {}", s);
        let parts = s.split('d').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(err_msg());
        }

        let num = parts[0].parse().map_err(|_| err_msg())?;
        let sides = parts[1].parse().map_err(|_| err_msg())?;

        Ok(Self::new(num, sides))
    }
}
