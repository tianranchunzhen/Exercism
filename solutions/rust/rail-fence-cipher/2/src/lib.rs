use itertools::Itertools;

pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        if self.0 == 1 {
            return text.to_string();
        }

        let mut rails = vec![Vec::new(); self.0];
        let rail_cycle = (0..self.0 - 1).chain((1..self.0).rev()).cycle();
        rail_cycle
            .zip(text.chars())
            .for_each(|(rail_idx, char)| rails[rail_idx].push(char));
        rails.iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        if self.0 == 1 {
            return cipher.to_string();
        }

        let mut rails = vec![Vec::new(); self.0];
        let rail_order = (0..self.0 - 1)
            .chain((1..self.0).rev())
            .cycle()
            .take(cipher.len());
        rail_order
            .clone()
            .sorted_unstable()
            .zip(cipher.chars())
            .for_each(|(rail_idx, char)| rails[rail_idx].push(char));

        let mut rails_iters = rails.into_iter().map(|rail| rail.into_iter()).collect_vec();
        rail_order
            .map(|rail_idx| rails_iters[rail_idx].next().unwrap())
            .collect::<String>()
    }
}
