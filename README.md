# fnv-rs

An implementation of the [Fowler–Noll–Vo hash function](http://www.isthe.com/chongo/tech/comp/fnv/index.html) ([FNV-1a](https://datatracker.ietf.org/doc/html/draft-eastlake-fnv-03)) - including 32, 64, 128, 256, 512, & 1024 bit variants.

## About

The FNV hash function provides a custom `Hasher` implementation that is more
efficient for smaller hash keys, along with other variants that can provide longer hash outputs.

[The Rust Standard Library documentation](http://www.isthe.com/chongo/tech/comp/fnv/index.html) states that while the default
`Hasher` implementation, SipHash, is good in many cases, it is notably slower
than other algorithms with short keys, such as when you have a map of integers
to other values.  In cases like these, [FNV is demonstrably faster](https://cglab.ca/~abeinges/blah/hash-rs/).

Its disadvantages are that it performs badly on larger inputs, and
provides no protection against collision attacks, where a malicious user
can craft specific keys designed to slow a hasher down. Thus, it is
important to profile your program to ensure that you are using small hash
keys, and be certain that your program could not be exposed to malicious
inputs (including being a networked server).

The Rust compiler itself uses FNV, as it is not worried about
denial-of-service attacks, and can assume that its inputs are going to be
small—a perfect use case for FNV.

## General Use

### Hash Method

```rust
use fnv_rs::{Fnv64, FnvHasher, FnvHashResult};

let hash = Fnv64::hash(b"Hash this!testing123");
println!("{}", hash);   // AD2808D0C15A663E
```

### Update Method

```rust
use fnv_rs::{Fnv64, FnvHasher, FnvHashResult};

let mut hasher = Fnv64::new();
hasher.update(b"Hash this!");
hasher.update(b"testing123");
let hash = hasher.finalize();
println!("{}", hash);               // AD2808D0C15A663E
println!("{:X}", hash);             // AD2808D0C15A663E
println!("{:x}", hash);             // ad2808d0c15a663e
println!("{}", hash.as_hex());      // AD2808D0C15A663E
println!("{:?}", hash.as_bytes());  // [173, 40, 8, 208, 193, 90, 102, 62]
```

## Using FNV in a HashMap

The `FnvHashMap` type alias is the easiest way to use the standard library’s
`HashMap` with FNV. (This uses FNV with a 64 bit output.)

```rust
use fnv_rs::FnvHashMap;

let mut map = FnvHashMap::default();
map.insert(1, "one");
map.insert(2, "two");

map = FnvHashMap::with_capacity_and_hasher(10, Default::default());
map.insert(1, "one");
map.insert(2, "two");
```

Note, the standard library’s `HashMap::new` and `HashMap::with_capacity`
are only implemented for the `RandomState` hasher, so using `Default` to
get the hasher is the next best option.

## Using FNV in a HashSet

Similarly, `FnvHashSet` is a type alias for the standard library’s `HashSet`
with FNV.

```rust
use fnv_rs::FnvHashSet;

let mut set = FnvHashSet::default();
set.insert(1);
set.insert(2);

set = FnvHashSet::with_capacity_and_hasher(10, Default::default());
set.insert(1);
set.insert(2);
```
