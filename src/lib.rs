#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
// #![no_std]

mod sealed;
mod self_or_mut;
mod self_or_ref;

pub use self_or_mut::SoM;
pub use self_or_ref::SoR;
