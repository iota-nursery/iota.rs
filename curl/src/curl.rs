use trytes::Trinary;

/// All implementations of `Curl` must implement this trait.
pub trait Curl {
    /// Absorb a `Trinary` into the sponge
    fn absorb(&mut self, trinary: Trinary);
    /// Squeeze the sponge and return a `Trinary` with `tritCount` trits
    fn squeeze(&mut self, trit_count: usize) -> Trinary;
}

/// An implementation of `Curl` that can reset its internal state.
pub trait CurlWithReset: Curl {
    /// Resets the sponge's internal state.
    fn reset(&mut self);
}
