use std::fmt::Display;

#[derive(Debug)]
pub struct Tick {
    tick: u64,
}

impl Tick {
    pub(crate) fn start() -> Self {
        Self { tick: 0 }
    }

    pub fn next(&mut self) {
        println!("Tick: {}", self.tick);
        self.tick += 1;
    }

    pub fn cur(&self) -> u64 {
        self.tick
    }
}

impl From<&Tick> for u64 {
    fn from(tick: &Tick) -> Self {
        tick.tick
    }
}

impl PartialOrd<u64> for &Tick {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        Some(self.tick.cmp(other))
    }
}

impl PartialOrd<&Tick> for u64 {
    fn partial_cmp(&self, other: &&Tick) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other.tick))
    }
}

impl PartialEq<u64> for &Tick {
    fn eq(&self, other: &u64) -> bool {
        self.tick == *other
    }
}

impl PartialEq<&Tick> for u64 {
    fn eq(&self, other: &&Tick) -> bool {
        *self == other.tick
    }
}

impl Display for Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tick {}", self.tick)
    }
}
