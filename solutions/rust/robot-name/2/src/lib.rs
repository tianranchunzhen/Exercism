use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_NAMES: usize = 26 * 26 * 1000;

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
        let count = COUNTER.fetch_add(1, Ordering::Relaxed);
        if count >= MAX_NAMES {
            panic!("All possible robot names have been used.");
        }

        let letters_part = count / 1000;

        let c2_offset = (letters_part % 26) as u8;
        let c1_offset = (letters_part / 26) as u8;

        let c1 = (b'A' + c1_offset) as char;
        let c2 = (b'A' + c2_offset) as char;

        format!("{}{}{:03}", c1, c2, count % 1000)
    }
}
