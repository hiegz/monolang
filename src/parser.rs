use crate::NumericType;

use chumsky::prelude::*;
use chumsky::*;

pub fn parser<'src>() -> impl Parser<'src, &'src str, ()> {
    empty()
}
