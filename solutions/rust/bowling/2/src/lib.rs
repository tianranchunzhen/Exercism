#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    pins: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second && self.pins.last().unwrap() + pins > 10) {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.score().is_some() {
            return Err(Error::GameComplete);
        }

        self.pins.push(pins);
        self.second = pins != 10 && !self.second;

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut idx = 0;

        for _ in 0..10 {
            let a = self.pins.get(idx).copied()?;
            let b = self.pins.get(idx + 1).copied()?;
            total += a + b;
            if a == 10 || a + b == 10 {
                total += self.pins.get(idx + 2).copied()?;
            }
            idx += if a == 10 { 1 } else { 2 };
        }

        Some(total)
    }
}
