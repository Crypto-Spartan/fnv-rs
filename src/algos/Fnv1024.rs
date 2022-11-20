use crate::{consts, FnvHashResult, FnvHasher};
use crypto_bigint::{UInt, U1024};

#[derive(Debug)]
pub struct FnvHasher1024(U1024);

impl Default for FnvHasher1024 {
    #[inline]
    fn default() -> FnvHasher1024 {
        FnvHasher1024(consts::FNV_OFFSET_1024)
    }
}

impl FnvHasher for FnvHasher1024 {
    #[inline]
    fn new() -> FnvHasher1024 {
        FnvHasher1024::default()
    }

    #[inline]
    fn finalize(&self) -> FnvHashResult {
        FnvHashResult::from_UInt(self.0)
    }

    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        let FnvHasher1024(mut hash) = *self;

        for byte in bytes.into_iter() {
            hash = hash ^ (UInt::<16>::from(*byte));
            hash = hash.wrapping_mul(&consts::FNV_PRIME_1024);
        }

        *self = FnvHasher1024(hash);
    }

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
        let mut hash = consts::FNV_OFFSET_1024;

        for byte in bytes.as_ref().into_iter() {
            hash = hash ^ (UInt::<16>::from(*byte));
            hash = hash.wrapping_mul(&consts::FNV_PRIME_1024);
        }

        FnvHashResult::from_UInt(hash)
    }
}