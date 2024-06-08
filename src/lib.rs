//!# Vec key value pair
//!A crate that adds drop in replacement for [`std::collections::HashMap`] and [`std::collections::HashSet`] that use linear search
//!instead of hashing.
//!
//!For extensive documentation and examples, see the original documentation, both [`map::VecMap`] and
//![`set::VecSet`] has identical API to [`std::collections::HashMap`] and  [`std::collections::HashSet`]
//!except for functions that interact with the hasher.
//!
//!For obvious reasons neither [`map::VecMap`] not [`set::VecSet`] use a hasher.

#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::unwrap_or_default
)]

///Contains [`map::VecMap`], drop in replacement for [`std::collections::HashMap`]
pub mod map;
///Contains [`set::VecSet`], drop in replacement for [`std::collections::HashSet`]
pub mod set;
