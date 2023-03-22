use crate::{consts, FnvHashResult, FnvHasher};
use crypto_bigint::{UInt, U256, U512, U1024};
use paste::paste;

macro_rules! create_fnvhasher {
    ($x:literal) => { paste! {
        
        #[derive(Debug)]
        pub struct [<FnvHasher $x>]([<u $x>]);

        impl Default for [<FnvHasher $x>] {
            #[inline]
            fn default() -> [<FnvHasher $x>] {
                [<FnvHasher $x>](consts::[<FNV_OFFSET_ $x>])
            }
        }

        impl FnvHasher for [<FnvHasher $x>] {

            #[inline]
            fn new() -> Self {
                [<FnvHasher $x>]::default()
            }

            #[inline]
            fn finalize(&self) -> FnvHashResult {
                FnvHashResult::[<from_u $x>](self.0)
            }

            #[inline]
            fn update(&mut self, bytes: &[u8]) {
                let mut hash = self.0;

                for byte in bytes.into_iter() {
                    hash = hash ^ (*byte as [<u $x>]);
                    hash = hash.wrapping_mul(consts::[<FNV_PRIME_ $x>]);
                }

                self.0 = hash;
            }

            #[inline]
            fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
                let mut hash = consts::[<FNV_OFFSET_ $x>];

                for byte in bytes.as_ref().into_iter() {
                    hash = hash ^ (*byte as [<u $x>]);
                    hash = hash.wrapping_mul(consts::[<FNV_PRIME_ $x>]);
                }

                FnvHashResult::[<from_u $x>](hash)
            }
        }
    }}
}


macro_rules! create_fnvhasher_bigint {
    ($x:literal) => { paste! {
        #[derive(Debug)]
        pub struct [<FnvHasher $x>]([<U $x>]);

        impl Default for [<FnvHasher $x>] {
            #[inline]
            fn default() -> [<FnvHasher $x>] {
                [<FnvHasher $x>](consts::[<FNV_OFFSET_ $x>])
            }
        }

        impl FnvHasher for [<FnvHasher $x>] {
            #[inline]
            fn new() -> [<FnvHasher $x>] {
                [<FnvHasher $x>]::default()
            }

            #[inline]
            fn finalize(&self) -> FnvHashResult {
                FnvHashResult::from_UInt(self.0)
            }

            #[inline]
            fn update(&mut self, bytes: &[u8]) {
                let mut hash = self.0;

                for byte in bytes.into_iter() {
                    hash = hash ^ (UInt::<{$x / 64}>::from(*byte));
                    hash = hash.wrapping_mul(&consts::[<FNV_PRIME_ $x>]);
                }

                self.0 = hash;
            }

            #[inline]
            fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
                let mut hash = consts::[<FNV_OFFSET_ $x>];

                for byte in bytes.as_ref().into_iter() {
                    hash = hash ^ (UInt::<{$x / 64}>::from(*byte));
                    hash = hash.wrapping_mul(&consts::[<FNV_PRIME_ $x>]);
                }

                FnvHashResult::from_UInt(hash)
            }
        }
    }}
}


pub(crate) use create_fnvhasher;
pub(crate) use create_fnvhasher_bigint;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macros() {
        create_fnvhasher!(32);
        create_fnvhasher_bigint!(256);
    }
}