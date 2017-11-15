//! Collection of cryptographic hash functions written in pure Rust.
//! This crate provides convenient re-exports from other crates. Additionally
//! it's a `no_std` crate, so it can be easily used in embedded applications.
//!
//! # Supported algorithms
//! * [BLAKE2](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2)
//! * [GOST94](https://en.wikipedia.org/wiki/GOST_(hash_function))
//!         (GOST R 34.11-94 and GOST 34.311-95) [weak]
//! * [MD4](https://en.wikipedia.org/wiki/MD2) [weak]
//! * [MD4](https://en.wikipedia.org/wiki/MD4) [weak]
//! * [MD5](https://en.wikipedia.org/wiki/MD5) [weak]
//! * [RIPEMD-160](https://en.wikipedia.org/wiki/RIPEMD)
//! * [SHA-1](https://en.wikipedia.org/wiki/SHA-1) [weak]
//! * [SHA-2](https://en.wikipedia.org/wiki/SHA-2)
//! * [SHA-3](https://en.wikipedia.org/wiki/SHA-3)
//! * [Streebog](https://en.wikipedia.org/wiki/Streebog) (GOST R 34.11-2012) [weak]
//! * [Whirlpool](https://en.wikipedia.org/wiki/Whirlpool_(cryptography))
//!
//! Algorithms marked by [weak] are not included by default. To use them enable
//! `include_weak` crate feature.
//!
//! # Usage
//!
//! ```rust
//! use crypto_hashes::digest::Digest;
//! use crypto_hashes::sha3::Sha3_256;
//!
//! // create a SHA3-256 object
//! let mut hasher = Sha3_256::default();
//!
//! // write input message
//! hasher.input(b"abc");
//!
//! // read result (this will consume hasher)
//! let out = hasher.result();
//!
//! assert_eq!(out[..], [0x3a, 0x98, 0x5d, 0xa7, 0x4f, 0xe2, 0x25, 0xb2,
//!                      0x04, 0x5c, 0x17, 0x2d, 0x6b, 0xd3, 0x90, 0xbd,
//!                      0x85, 0x5f, 0x08, 0x6e, 0x3e, 0x9d, 0x52, 0x5b,
//!                      0x46, 0xbf, 0xe2, 0x45, 0x11, 0x43, 0x15, 0x32]);
//! ```
#![no_std]
pub extern crate blake2;
#[cfg(feature = "include_weak")]
pub extern crate gost94;
pub extern crate groestl;
#[cfg(feature = "include_weak")]
pub extern crate md2;
#[cfg(feature = "include_weak")]
pub extern crate md4;
#[cfg(feature = "include_weak")]
pub extern crate md5;
pub extern crate ripemd160;
#[cfg(feature = "include_weak")]
pub extern crate sha1;
pub extern crate sha2;
pub extern crate sha3;
#[cfg(feature = "include_weak")]
pub extern crate streebog;
pub extern crate whirlpool;

pub extern crate digest;
