

//use std::default::Default;
use std::fmt;
use core::hash::{Hasher, BuildHasherDefault};
use std::collections::{HashMap, HashSet};
//use core::default::Default;
//use core::hash::{Hasher, BuildHasherDefault};
use crypto_bigint::{UInt, U256, U512, U1024};
use paste::paste;

mod consts;
mod macros;

#[cfg(test)]
mod tests;

pub trait FnvHasher {
    fn new() -> Self;
    fn update(&mut self, bytes: &[u8]);
    fn finalize(&self) -> FnvHashResult;
    fn hash<T: AsRef<[u8]>>(data: T) -> FnvHashResult;
}


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
    
    pub(crate) fn from_UInt<T>(bigint: T) -> Self 
    where
        T: crypto_bigint::prelude::Encoding,
    {
        FnvHashResult(Box::from(bigint.to_be_bytes().as_ref()))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_hex(&self) -> String {
        hex::encode(&self.0).to_uppercase()
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


macros::create_fnvhasher!(32);
macros::create_fnvhasher!(64);
macros::create_fnvhasher!(128);
macros::create_fnvhasher_bigint!(256);
macros::create_fnvhasher_bigint!(512);
macros::create_fnvhasher_bigint!(1024);

impl Hasher for FnvHasher64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

pub use FnvHasher32 as Fnv32;
pub use FnvHasher64 as Fnv64;
pub use FnvHasher128 as Fnv128;
pub use FnvHasher256 as Fnv256;
pub use FnvHasher512 as Fnv512;
pub use FnvHasher1024 as Fnv1024;


/// A builder for default FNV hashers.
pub type FnvBuildHasher = BuildHasherDefault<Fnv64>;

/// A `HashMap` using a default FNV hasher.
pub type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;

/// A `HashSet` using a default FNV hasher.
pub type FnvHashSet<T> = HashSet<T, FnvBuildHasher>;