//! Implementations of the `CharacterCache` trait.
//!
//! Enabled through Cargo features.
//!
//! ### RustType
//!
//! Add the following to "Cargo.toml":
//!
//! ```ignore
//! [dependencies.piston2d-graphics]
//! version = "*"
//! features = ["glyph_cache_rusttype"]
//! ```

#[cfg(feature = "glyph_cache_rusttype")]
pub mod rusttype;
