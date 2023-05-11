//! # Serde-teamspeak-querystring
//! serde-teamspeak-querystring is a deserializer for teamspeak query strings.
//!
//! To see what's supported and what's not, please take a look the tests in the main repo.
//!

mod de;
mod error;

pub use de::{from_bytes, from_str, escape};
pub use error::{Error, ErrorKind};
