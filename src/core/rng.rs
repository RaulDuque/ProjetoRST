//! Small deterministic random number generator (SplitMix64).
//!
//! Pure module: no Bevy types. Used for damage rolls, drops and AI jitter.
//! A seeded generator keeps gameplay reproducible in tests and replays, and
//! avoids adding an external crate for a few floats per frame.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameRng {
    state: u64,
}

impl GameRng {
    pub fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform float in `[0, 1)`.
    pub fn next_f32(&mut self) -> f32 {
        // 24 random mantissa bits.
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floats_are_in_unit_range_and_deterministic() {
        let mut a = GameRng::seeded(7);
        let mut b = GameRng::seeded(7);
        for _ in 0..1000 {
            let x = a.next_f32();
            assert!((0.0..1.0).contains(&x));
            assert_eq!(x, b.next_f32());
        }
    }
}
