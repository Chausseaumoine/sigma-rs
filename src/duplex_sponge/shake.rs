//! SHAKE-based duplex sponge implementation
//!
//! This module implements a duplex sponge construction using SHAKE128.

use crate::duplex_sponge::DuplexSpongeInterface;
use sha3::{
    digest::{ExtendableOutput, Reset, Update},
    Shake128,
};

/// Duplex sponge construction using SHAKE128.
#[derive(Clone, Debug)]
pub struct ShakeDuplexSponge(Shake128);

impl DuplexSpongeInterface for ShakeDuplexSponge {
    fn new(iv: [u8; 32]) -> Self {
        let mut hasher = Shake128::default();
        hasher.update(&iv);
        Self(hasher)
    }

    fn absorb(&mut self, input: &[u8]) {
        self.0.update(input);
    }

    fn squeeze(&mut self, length: usize) -> Vec<u8> {
        let mut output = vec![0u8; length];
        self.0.clone().finalize_xof_into(&mut output);
        output
    }

    fn ratchet(&mut self) {
        let mut output = [0u8; 32];
        self.0.clone().finalize_xof_into(&mut output);
        self.0.reset();
        self.0.update(&output);
    }
}
