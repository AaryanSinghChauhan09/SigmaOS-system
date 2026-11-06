//! `crypto` — minimal crypto shim for modules referencing
//! `crate::crypto::entropy`.

pub mod entropy {
    /// Fill `buf` with deterministic pseudo-entropy (xorshift64*).
    /// Model implementation — production uses RDRAND/RDSEED or
    /// SigmaOS-entropy pool.
    pub fn get_entropy_bytes(buf: &mut [u8]) {
        let mut s = 0x9e37_79b9_7f4a_7c15u64;
        for chunk in buf.chunks_mut(8) {
            s ^= s >> 12; s ^= s << 25; s ^= s >> 27;
            let v = s.wrapping_mul(0x2545_f491_4f6c_dd1d).to_le_bytes();
            let n = chunk.len().min(8);
            chunk[..n].copy_from_slice(&v[..n]);
        }
    }
}
