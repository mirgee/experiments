extern crate once_cell;
extern crate regex;

mod error;
mod parsed_did;
mod utils;

use std::ops::Range;

type DIDRange = Range<usize>;

pub use error::ParseError;
pub use parsed_did::ParsedDID;
pub use utils::validate::{is_valid_did, is_valid_did_url};
