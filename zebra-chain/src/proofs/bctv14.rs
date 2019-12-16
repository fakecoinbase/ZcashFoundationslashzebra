use std::fmt;

/// An encoding of a BCTV14 proof, as used in Zcash.
pub struct Bctv14Proof(pub [u8; 296]);

impl fmt::Debug for Bctv14Proof {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Bctv14Proof")
            .field(&hex::encode(&self.0[..]))
            .finish()
    }
}

// These impls all only exist because of array length restrictions.

impl Copy for Bctv14Proof {}

impl Clone for Bctv14Proof {
    fn clone(&self) -> Self {
        let mut bytes = [0; 296];
        bytes[..].copy_from_slice(&self.0[..]);
        Self(bytes)
    }
}

impl PartialEq for Bctv14Proof {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Eq for Bctv14Proof {}