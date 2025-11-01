#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    pins: Vec<u16>,
    frame: u16,
    new_frame: bool,
    count_strike_before_ten: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            new_frame: true,
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.frame {
            0..=8 => {
                if self.new_frame {
                    self.pins.push(pins);
                    match pins {
                        10 => {
                            self.frame += 1;
                            self.count_strike_before_ten += 1
                        }
                        _ => self.new_frame = false,
                    }
                } else if self.pins.last().unwrap() + pins <= 10 {
                    self.pins.push(pins);
                    self.frame += 1;
                    self.new_frame = true;
                } else {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            9 => {
                if self.pins.len() == 18 - self.count_strike_before_ten {
                    self.pins.push(pins);
                    match pins {
                        10 => (),
                        _ => self.new_frame = false,
                    }
                } else if self.pins.len() == 19 - self.count_strike_before_ten {
                    if self.new_frame {
                        self.pins.push(pins);
                        match pins {
                            10 => (),
                            _ => self.new_frame = false,
                        }
                    } else if self.pins.last().unwrap() + pins == 10 {
                        self.pins.push(pins);
                        self.new_frame = true;
                    } else if self.pins.last().unwrap() + pins < 10 {
                        self.pins.push(pins);
                        self.frame += 1;
                    } else {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                } else if self.pins.len() == 20 - self.count_strike_before_ten {
                    if self.new_frame || self.pins.last().unwrap() + pins <= 10 {
                        self.pins.push(pins);
                        self.frame += 1;
                    } else {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
            }
            _ => return Err(Error::GameComplete),
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame < 10 {
            return None;
        }

        let mut total_score = self
            .pins
            .iter()
            .skip(18 - self.count_strike_before_ten)
            .sum();
        let mut idx_iter = 0..18 - self.count_strike_before_ten;
        while let Some(idx) = idx_iter.next() {
            if self.pins[idx] == 10 {
                total_score += 10 + self.pins[idx + 1] + self.pins[idx + 2];
            } else {
                if self.pins[idx] + self.pins[idx + 1] == 10 {
                    total_score += 10 + self.pins[idx + 2];
                } else {
                    total_score += self.pins[idx] + self.pins[idx + 1]
                }
                idx_iter.next();
            }
        }

        Some(total_score)
    }
}
