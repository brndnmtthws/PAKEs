//! [Secure Remote Password][1] (SRP) protocol implementation.
//!
//! This implementation is generic over hash functions using
//! [`Digest`](https://docs.rs/digest) trait, so you will need to choose a hash
//! function, e.g. `Sha256` from [`sha2`](https://crates.io/crates/sha2) crate.
//! Additionally this crate allows to use a specialized password hashing
//! algorithm for private key computation instead of method described in the
//! SRP literature.
//!
//! Compatibility with over implementations was not yet tested.
//!
//! # Usage
//! Add `srp` dependecy to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rand = "0.3"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate srp;
//! ```
//!
//! Next read documentation for [`client`](client/index.html) and
//! [`server`](server/index.html) modules.
//!
//! # Algorithm description
//! Here we briefly describe implemented algroithm. For additionall information
//! refer to SRP literature. All arithmetic is done modulo `N`, where `N` is a
//! large safe prime (`N = 2q+1`, where `q` is prime). Additionally `g` MUST be
//! a generator modulo `N`. It's STRONGLY recommended to use SRP parameters
//! provided by this crate in the [`groups`](groups/index.html) module.
//!
//! |       Client           |   Data transfer   |      Server                     |
//! |------------------------|-------------------|---------------------------------|
//! |`a_pub = g^a`           | — `a_pub`, `I` —> | (lookup `s`, `v` for given `I`) |
//! |`x = PH(P, s)`          | <— `b_pub`, `s` — | `b_pub = k*v + g^b`             |
//! |`u = H(a_pub ‖ b_pub)`  |                   | `u = H(a_pub ‖ b_pub)`          |
//! |`s = (b_pub - k*g^x)^(a+u*x)` |             | `S = (b_pub - k*g^x)^(a+u*x)`   |
//! |`K = H(s)`              |                   | `K = H(s)`                      |
//! |`M1 = H(A ‖ B ‖ K)`     |     — `M1` —>     | (verify `M1`)                   |
//! |(verify `M2`)           |    <— `M2` —      | `M2 = H(A ‖ M1 ‖ K)`            |
//!
//! Variables and notations have the following
//! meaning:
//!
//! - `I` — user identity (username)
//! - `P` — user password
//! - `H` — one-way hash function
//! - `PH` — password hashing algroithm, in the RFC 5054 described as
//! `H(s ‖ H(I ‖ ":" ‖ P))`
//! - `^` — (modular) exponentiation
//! - `‖` — concatenation
//! - `x` — user private key
//! - `s` — salt generated by user and stored on the server
//! - `v` — password verifier equal to `g^x` and stored on the server
//! - `a`, `b` — secret ephemeral values (at least 256 bits in length)
//! - `A`, `B` — Public ephemeral values
//! - `u` — scrambling parameter
//! - `k` — multiplier parameter (`k = H(N || g)` in SRP-6a)
//!
//! [1]: https://en.wikipedia.org/wiki/Secure_Remote_Password_protocol
//! [2]: https://tools.ietf.org/html/rfc5054
extern crate num;
extern crate digest;
extern crate generic_array;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate sha_1;

mod tools;
pub mod client;
pub mod server;
pub mod types;
pub mod groups;