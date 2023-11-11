#![allow(non_upper_case_globals)]
pub mod offsets;
pub mod util;
pub mod singletons;
pub mod consts;
pub mod ext;

#[macro_use]
extern crate modular_bitfield;

pub use hdr_macros::{
    export,
    import,
    import_noreturn,
    hash40
};

pub use hdr_macros as macros;
