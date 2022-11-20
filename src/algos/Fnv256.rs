use crate::{consts, FnvHashResult, FnvHasher};
use crypto_bigint::{UInt, U256};

#[derive(Debug)]
pub struct FnvHasher256(U256);

impl Default for FnvHasher256 {
    #[inline]
    fn default() -> FnvHasher256 {
        FnvHasher256(consts::FNV_OFFSET_256)
    }
}

impl FnvHasher for FnvHasher256 {
    #[inline]
    fn new() -> FnvHasher256 {
        FnvHasher256::default()
    }

    #[inline]
    fn finalize(&self) -> FnvHashResult {
        FnvHashResult::from_UInt(self.0)
    }

    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        let FnvHasher256(mut hash) = *self;

        for byte in bytes.into_iter() {
            hash = hash ^ (UInt::<4>::from(*byte));
            hash = hash.wrapping_mul(&consts::FNV_PRIME_256);
        }

        *self = FnvHasher256(hash);
    }

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
        let mut hash = consts::FNV_OFFSET_256;

        for byte in bytes.as_ref().into_iter() {
            hash = hash ^ (UInt::<4>::from(*byte));
            hash = hash.wrapping_mul(&consts::FNV_PRIME_256);
        }

        FnvHashResult::from_UInt(hash)
    }
}