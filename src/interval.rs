pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surronds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: -f64::INFINITY,
};
const UNIVERSE: Interval = Interval {
    min: -f64::INFINITY,
    max: -f64::INFINITY,
};
