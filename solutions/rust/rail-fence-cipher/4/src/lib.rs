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
        (0..self.0 - 1)
            .chain((1..self.0).rev())
            .cycle()
            .zip(text.chars())
            .for_each(|(rail_idx, char)| rails[rail_idx].push(char));
        rails.iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        if self.0 == 1 {
            return cipher.to_string();
        }

        let mut indexes: Vec<_> = (0..self.0 - 1)
            .chain((1..self.0).rev())
            .cycle()
            .zip(1..)
            .take(cipher.len())
            .collect();
        indexes.sort_by_key(|&(rail, _)| rail);

        let mut pairs: Vec<_> = cipher
            .chars()
            .zip(indexes)
            .map(|(c, (_, i))| (i, c))
            .collect();
        pairs.sort_by_key(|&(i, _)| i);
        pairs.iter().map(|(_, c)| c).collect()
    }
}
