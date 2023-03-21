use crate::{consts, FnvHashResult, FnvHasher};

#[derive(Debug)]
pub struct FnvHasher128(u128);

impl Default for FnvHasher128 {
    #[inline]
    fn default() -> FnvHasher128 {
        FnvHasher128(consts::FNV_OFFSET_128)
    }
}

impl FnvHasher for FnvHasher128 {
    #[inline]
    fn new() -> Self {
        FnvHasher128::default()
    }

    #[inline]
    fn finalize(&self) -> FnvHashResult {
        FnvHashResult::from_u128(self.0)
    }

    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        let mut hash = self.0;

        for byte in bytes.into_iter() {
            hash = hash ^ (*byte as u128);
            hash = hash.wrapping_mul(consts::FNV_PRIME_128);
        }

        self.0 = hash;
    }

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
        let mut hash = consts::FNV_OFFSET_128;

        for byte in bytes.as_ref().into_iter() {
            hash = hash ^ (*byte as u128);
            hash = hash.wrapping_mul(consts::FNV_PRIME_128);
        }

        FnvHashResult::from_u128(hash)
    }
}