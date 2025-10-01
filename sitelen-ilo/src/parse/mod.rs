use std::fmt::Display;

use nom::Err;
use nom_locate::LocatedSpan;

pub(crate) mod error;
pub(crate) mod object;
pub(crate) mod util;

pub(crate) type Span<'a> = LocatedSpan<&'a str>;

