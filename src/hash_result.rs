use std::fmt;
use hex;

/// A specialized result type for FNV operations.
pub struct FnvHashResult(Box<[u8]>);

impl FnvHashResult {
    pub(crate) fn from_u32(i: u32) -> Self {
        FnvHashResult(Box::from(i.to_be_bytes()))
    }

    pub(crate) fn from_u64(i: u64) -> Self {
        FnvHashResult(Box::from(i.to_be_bytes()))
    }

    pub(crate) fn from_u128(i: u128) -> Self {
        FnvHashResult(Box::from(i.to_be_bytes()))
    }

    #[cfg(feature = "bigint")]
    pub(crate) fn from_bigint<T>(bigint: T) -> Self
    where
        T: crypto_bigint::prelude::Encoding,
    {
        FnvHashResult(Box::from(bigint.to_be_bytes().as_ref()))
    }

    /// Returns a byte slice of this `FnvHashResult`'s contents.
    ///
    /// # Example:
    ///
    /// ```
    /// let hash = Fnv64::hash(bytes);
    /// let hash_bytes = hash.as_bytes();
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns an all-caps hex `String` of this `FnvHashResult`'s contents.
    ///
    /// # Example:
    ///
    /// ```
    /// let hash = Fnv64::hash(bytes);
    /// let hash_hex = hash.as_hex();
    /// ```
    pub fn as_hex(&self) -> String {
        hex::encode(&self.0).to_uppercase()
    }

    /// Returns the length of this `FnvHashResult`'s underlying byte slice.
    ///
    /// # Example:
    ///
    /// ```
    /// let hash = Fnv64::hash(bytes);
    /// let hash_byte_count = hash.len();
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for FnvHashResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl fmt::LowerHex for FnvHashResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

impl fmt::UpperHex for FnvHashResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl fmt::Debug for FnvHashResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FnvHashResult")
            .field(&self.as_hex())
            .finish()
    }
}
