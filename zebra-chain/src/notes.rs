//! Note encryption types.
mod memo;
pub mod sapling;
pub mod sprout;

/// The randomness used in the Pedersen Hash for note commitment.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoteCommitmentRandomness(pub [u8; 32]);
