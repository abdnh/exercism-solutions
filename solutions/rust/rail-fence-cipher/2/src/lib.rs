pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        let gap = (self.0 - 2) * 2 + 1;
        let mut buckets: Vec<String> = vec![String::new(); self.0];
        text.chars().enumerate().for_each(|(i, chr)| {
            buckets[self.0 - 1 - (self.0 - 1).abs_diff(i % (gap + 1))].push(chr);
        });

        buckets.concat()
    }

    pub fn decode(&self, cipher: &str) -> String {
        // TODO: find a more efficient method
        let mut text = cipher.to_string();
        loop {
            let encoded = self.encode(&text).to_string();
            if encoded == cipher {
                return text;
            }
            text = encoded;
        }
    }
}
