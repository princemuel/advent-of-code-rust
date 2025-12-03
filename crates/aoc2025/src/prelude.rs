//! Convenience prelude for internal modules.
//!
//! modules can `use crate::prelude::*;` to pull in common
//! types and imports and avoid long `use` lists.

pub use core::error::Error;
pub use core::str::FromStr;
pub use std::io::Read;
pub use std::path::{Path, PathBuf};
pub use std::{fmt, fs, io};

/// Read all data from standard input into a string.
///
/// This is a small helper used by the generated day binaries.
pub fn read_input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read buf");
    buf.trim().to_string()
}
