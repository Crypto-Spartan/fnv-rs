#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod consts;
mod macros;
mod hash_result;
#[cfg(test)]
mod tests;


#[cfg(feature = "bigint")]
use crypto_bigint::{UInt, U256, U512, U1024};

use paste::paste;
use std::collections::{HashMap, HashSet};
use core::hash::{Hasher, BuildHasherDefault};

pub use hash_result::FnvHashResult;


pub trait FnvHasher {
    fn new() -> Self;
    fn update(&mut self, bytes: &[u8]);
    fn finalize(&self) -> FnvHashResult;
    fn hash<T: AsRef<[u8]>>(data: T) -> FnvHashResult;
}


macros::create_fnvhasher!(32);
macros::create_fnvhasher!(64);
macros::create_fnvhasher!(128);
macros::create_fnvhasher_bigint!(256);
macros::create_fnvhasher_bigint!(512);
macros::create_fnvhasher_bigint!(1024);


impl Hasher for Fnv64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

/// A builder for default FNV hashers.
pub type FnvBuildHasher = BuildHasherDefault<Fnv64>;

/// A `HashMap` using a default FNV hasher.
pub type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;

/// A `HashSet` using a default FNV hasher.
pub type FnvHashSet<T> = HashSet<T, FnvBuildHasher>;
