extern crate once_cell;
extern crate regex;

mod error;
mod parsed_did;
mod parsing;

use std::ops::Range;

type DIDRange = Range<usize>;

pub use error::ParseError;
pub use parsed_did::ParsedDID;
