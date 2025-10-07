use rand;

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::gen_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::gen_name()
    }

    fn gen_name() -> String {
        let bytes: [u8; 5] = [
            rand::random_range(65..=90),
            rand::random_range(65..=90),
            rand::random_range(48..=57),
            rand::random_range(48..=57),
            rand::random_range(48..=57),
        ];
        bytes.iter().map(|b| *b as char).collect()
    }
}
