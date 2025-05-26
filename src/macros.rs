macro_rules! create_fnvhasher {
    ($x:literal) => { paste! {

        #[doc = "A FNV-1a hasher that produces a " $x "-bit output."]
        ///
        /// # Examples:
        ///
        /// ```
        #[doc = "let mut hasher = Fnv" $x "::new();"]
        /// hasher.update(bytes);
        /// hasher.finalize();
        /// ```
        ///
        /// OR
        ///
        /// ```
        #[doc = "let hash = Fnv" $x "::hash(bytes);"]
        /// ```
        #[derive(Debug)]
        pub struct [<Fnv $x>]([<u $x>]);

        impl Default for [<Fnv $x>] {
            #[inline]
            fn default() -> [<Fnv $x>] {
                [<Fnv $x>](consts::[<FNV_OFFSET_ $x>])
            }
        }

        impl FnvHasher for [<Fnv $x>] {

            #[doc = "Creates a new default `Fnv" $x "` object."]
            ///
            /// # Example:
            ///
            /// ```
            #[doc = "let hasher = Fnv" $x "::new();"]
            /// ```
            #[inline]
            fn new() -> Self {
                [<Fnv $x>]::default()
            }

            #[doc = "Incrementally update the `Fnv" $x "` object."]
            ///
            /// This method is best used when you need to update the hasher multiple times.
            /// If you only need to hash a single piece of data, consider using the [`hash`] method instead.
            ///
            #[doc = "[`hash`]: Fnv" $x "::hash"]
            ///
            /// # Example:
            ///
            /// ```
            /// hasher.update(bytes);
            /// ```
            #[inline]
            fn update(&mut self, bytes: &[u8]) {
                let mut hash = self.0;

                for byte in bytes.into_iter() {
                    hash = hash ^ (*byte as [<u $x>]);
                    hash = hash.wrapping_mul(consts::[<FNV_PRIME_ $x>]);
                }

                self.0 = hash;
            }

            #[doc = "Finalize the `Fnv" $x "` object."]
            ///
            /// # Example:
            ///
            /// ```
            /// hasher.finalize();
            /// ```
            #[inline]
            fn finalize(&self) -> FnvHashResult {
                FnvHashResult::[<from_u $x>](self.0)
            }

            #[doc = "One-time use of the `Fnv" $x "` object."]
            ///
            /// Using this method is shorthand for the following:
            ///
            /// ```
            #[doc = "let mut hasher = Fnv" $x "::new();"]
            /// hasher.update(bytes);
            /// hasher.finalize();
            /// ```
            /// # Example:
            ///
            /// ```
            #[doc = "let hash = Fnv" $x "::hash(bytes);"]
            /// ```
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

        impl From<[<Fnv $x>]> for [<u $x>] {
            fn from(value: [<Fnv $x>]) -> Self {
                value.0
            }
        }
    }}
}

macro_rules! create_fnvhasher_bigint {
    ($x:literal) => { paste! {

        #[doc = "A FNV-1a hasher that produces a " $x "-bit output."]
        ///
        /// # Examples:
        ///
        /// ```
        #[doc = "let mut hasher = Fnv" $x "::new();"]
        /// hasher.update(bytes);
        /// hasher.finalize();
        /// ```
        ///
        /// OR
        ///
        /// ```
        #[doc = "let hash = Fnv" $x "::hash(bytes);"]
        /// ```
        #[derive(Debug)]
        #[cfg(feature = "bigint")]
        pub struct [<Fnv $x>]([<U $x>]);

        #[cfg(feature = "bigint")]
        impl Default for [<Fnv $x>] {
            #[inline]
            fn default() -> [<Fnv $x>] {
                [<Fnv $x>](consts::[<FNV_OFFSET_ $x>])
            }
        }

        #[cfg(feature = "bigint")]
        impl FnvHasher for [<Fnv $x>] {

            #[doc = "Creates a new default `Fnv" $x "` object."]
            ///
            /// # Example:
            ///
            /// ```
            #[doc = "let hasher = Fnv" $x "::new();"]
            /// ```
            #[inline]
            fn new() -> [<Fnv $x>] {
                [<Fnv $x>]::default()
            }

            #[doc = "Incrementally update the `Fnv" $x "` object."]
            ///
            /// This method is best used when you need to update the hasher multiple times.
            /// If you only need to hash a single piece of data, consider using the [`hash`] method instead.
            ///
            #[doc = "[`hash`]: Fnv" $x "::hash"]
            ///
            /// # Example:
            ///
            /// ```
            /// hasher.update(bytes);
            /// ```
            #[inline]
            fn update(&mut self, bytes: &[u8]) {
                let mut hash = self.0;

                for byte in bytes.into_iter() {
                    hash = hash ^ (Uint::from_u8(*byte));
                    hash = hash.wrapping_mul(&consts::[<FNV_PRIME_ $x>]);
                }

                self.0 = hash;
            }

            #[doc = "Finalize the `Fnv" $x "` object."]
            ///
            /// # Example:
            ///
            /// ```
            /// hasher.finalize();
            /// ```
            #[inline]
            fn finalize(&self) -> FnvHashResult {
                FnvHashResult::from_bigint(self.0)
            }

            #[doc = "One-time use of the `Fnv" $x "` object."]
            ///
            /// Using this method is shorthand for the following:
            ///
            /// ```
            #[doc = "let mut hasher = Fnv" $x "::new();"]
            /// hasher.update(bytes);
            /// hasher.finalize();
            /// ```
            /// # Example:
            ///
            /// ```
            #[doc = "let hash = Fnv" $x "::hash(bytes);"]
            /// ```
            #[inline]
            fn hash<T: AsRef<[u8]>>(bytes: T) -> FnvHashResult {
                let mut hash = consts::[<FNV_OFFSET_ $x>];

                for byte in bytes.as_ref().into_iter() {
                    hash = hash ^ (Uint::from_u8(*byte));
                    hash = hash.wrapping_mul(&consts::[<FNV_PRIME_ $x>]);
                }

                FnvHashResult::from_bigint(hash)
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
