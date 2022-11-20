#![allow(dead_code)]

use crypto_bigint::{U256, U512, U1024};

// FNV Primes for primitive types
pub const FNV_PRIME_32: u32 = 0x1000193;
pub const FNV_PRIME_64: u64 = 0x100_000001B3;
pub const FNV_PRIME_128: u128 = 0x1000000_00000000_0000013B;

// FNV Primes for larger numbers
pub const FNV_PRIME_256: U256 = U256::from_be_slice(&[
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x63u8
]);
pub const FNV_PRIME_512: U512 = U512::from_be_slice(&[
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x57u8
]);
pub const FNV_PRIME_1024: U1024 = U1024::from_be_slice(&[
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x01u8, 0x8Du8
]);


// FNV offset basis for primitive types
pub const FNV_OFFSET_32: u32 = 0x811C9DC5;
pub const FNV_OFFSET_64: u64 = 0xCBF29CE4_84222325;
pub const FNV_OFFSET_128: u128 = 0x6C62272E_07BB0142_62B82175_6295C58D;

// FNV offset basis for larger numbers
pub const FNV_OFFSET_256: U256 =  U256::from_be_slice(&[
    0xDDu8, 0x26u8, 0x8Du8, 0xBCu8, 0xAAu8, 0xC5u8, 0x50u8, 0x36u8,
    0x2Du8, 0x98u8, 0xC3u8, 0x84u8, 0xC4u8, 0xE5u8, 0x76u8, 0xCCu8,
    0xC8u8, 0xB1u8, 0x53u8, 0x68u8, 0x47u8, 0xB6u8, 0xBBu8, 0xB3u8,
    0x10u8, 0x23u8, 0xB4u8, 0xC8u8, 0xCAu8, 0xEEu8, 0x05u8, 0x35u8
]);
pub const FNV_OFFSET_512: U512 = U512::from_be_slice(&[
    0xB8u8, 0x6Du8, 0xB0u8, 0xB1u8, 0x17u8, 0x1Fu8, 0x44u8, 0x16u8,
    0xDCu8, 0xA1u8, 0xE5u8, 0x0Fu8, 0x30u8, 0x99u8, 0x90u8, 0xACu8,
    0xACu8, 0x87u8, 0xD0u8, 0x59u8, 0xC9u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x0Du8, 0x21u8,
    0xE9u8, 0x48u8, 0xF6u8, 0x8Au8, 0x34u8, 0xC1u8, 0x92u8, 0xF6u8,
    0x2Eu8, 0xA7u8, 0x9Bu8, 0xC9u8, 0x42u8, 0xDBu8, 0xE7u8, 0xCEu8,
    0x18u8, 0x20u8, 0x36u8, 0x41u8, 0x5Fu8, 0x56u8, 0xE3u8, 0x4Bu8,
    0xACu8, 0x98u8, 0x2Au8, 0xACu8, 0x4Au8, 0xFEu8, 0x9Fu8, 0xD9u8
]);
pub const FNV_OFFSET_1024: U1024 = U1024::from_be_slice(&[
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x5Fu8, 0x7Au8, 0x76u8, 0x75u8, 0x8Eu8, 0xCCu8, 0x4Du8,
    0x32u8, 0xE5u8, 0x6Du8, 0x5Au8, 0x59u8, 0x10u8, 0x28u8, 0xB7u8,
    0x4Bu8, 0x29u8, 0xFCu8, 0x42u8, 0x23u8, 0xFDu8, 0xADu8, 0xA1u8,
    0x6Cu8, 0x3Bu8, 0xF3u8, 0x4Eu8, 0xDAu8, 0x36u8, 0x74u8, 0xDAu8,
    0x9Au8, 0x21u8, 0xD9u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
    0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x04u8, 0xC6u8, 0xD7u8,
    0xEBu8, 0x6Eu8, 0x73u8, 0x80u8, 0x27u8, 0x34u8, 0x51u8, 0x0Au8,
    0x55u8, 0x5Fu8, 0x25u8, 0x6Cu8, 0xC0u8, 0x05u8, 0xAEu8, 0x55u8,
    0x6Bu8, 0xDEu8, 0x8Cu8, 0xC9u8, 0xC6u8, 0xA9u8, 0x3Bu8, 0x21u8,
    0xAFu8, 0xF4u8, 0xB1u8, 0x6Cu8, 0x71u8, 0xEEu8, 0x90u8, 0xB3u8
]);