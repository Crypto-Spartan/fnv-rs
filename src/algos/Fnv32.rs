use crate::{consts, FnvHashResult, FnvHasher};

#[derive(Debug)]
pub struct FnvHasher32(u32);

impl Default for FnvHasher32 {
    #[inline]
    fn default() -> FnvHasher32 {
        FnvHasher32(consts::FNV_OFFSET_32)
    }
}

impl FnvHasher for FnvHasher32 {

    #[inline]
    fn new() -> Self {
        FnvHasher32::default()
    }

    #[inline]
    fn finalize(&self) -> FnvHashResult {
        FnvHashResult::from_u32(self.0)
    }

    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        let FnvHasher32(mut hash) = *self;

        for byte in bytes.into_iter() {
            hash = hash ^ (*byte as u32);
            hash = hash.wrapping_mul(consts::FNV_PRIME_32);
        }

        *self = FnvHasher32(hash);
    }

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
        let mut hash = consts::FNV_OFFSET_32;

        for byte in bytes.as_ref().into_iter() {
            hash = hash ^ (*byte as u32);
            hash = hash.wrapping_mul(consts::FNV_PRIME_32);
        }

        FnvHashResult::from_u32(hash)
    }
}