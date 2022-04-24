#![warn(missing_docs)]
//! `partialzip` is a crate to download or list single files from online zip archives
//! by saving memory and download times.
//!
//! It fetches first zip data structures and then download only the
//! relevant parts of the archive and proceed to decompress it.
/// Empty module for now
pub mod commons;
/// Core module for the partialzip
pub mod partzip;
/// Small utilities mostly for urls
pub mod utils;
