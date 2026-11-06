//! `klib` — shared kernel/userland library shim.
//!
//! Several generated modules were written against a `crate::klib::*`
//! path that was never created. This module provides the expected
//! surface as aliases over `std`/`alloc` plus a small deterministic RNG.

pub mod btreemap {
    pub use std::collections::BTreeMap;
}
pub mod path {
    pub type PathBuf = std::string::String;
}
pub mod rng {
    /// Deterministic xorshift64* RNG — model entropy source.
    pub struct SigmaRng(u64);
    impl SigmaRng {
        pub fn new(seed: u64) -> Self { Self(seed | 1) }
        pub fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12; x ^= x << 25; x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_f491_4f6c_dd1d)
        }
        pub fn fill_bytes(&mut self, buf: &mut [u8]) {
            for chunk in buf.chunks_mut(8) {
                let v = self.next_u64().to_le_bytes();
                let n = chunk.len().min(8);
                chunk[..n].copy_from_slice(&v[..n]);
            }
        }
    }
}
pub use std::collections::HashMap;
