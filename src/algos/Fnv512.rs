use crate::{consts, FnvHashResult, FnvHasher};
use crypto_bigint::{UInt, U512};

#[derive(Debug)]
pub struct FnvHasher512(U512);

impl Default for FnvHasher512 {
    #[inline]
    fn default() -> FnvHasher512 {
        FnvHasher512(consts::FNV_OFFSET_512)
    }
}

impl FnvHasher for FnvHasher512 {
    #[inline]
    fn new() -> FnvHasher512 {
        FnvHasher512::default()
    }

    #[inline]
    fn finalize(&self) -> FnvHashResult {
        FnvHashResult::from_UInt(self.0)
    }

    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        let mut hash = self.0;

        for byte in bytes.into_iter() {
            hash = hash ^ (UInt::<8>::from(*byte));
            hash = hash.wrapping_mul(&consts::FNV_PRIME_512);
        }

        self.0 = hash;
    }

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
        let mut hash = consts::FNV_OFFSET_512;

        for byte in bytes.as_ref().into_iter() {
            hash = hash ^ (UInt::<8>::from(*byte));
            hash = hash.wrapping_mul(&consts::FNV_PRIME_512);
        }

        FnvHashResult::from_UInt(hash)
    }
}