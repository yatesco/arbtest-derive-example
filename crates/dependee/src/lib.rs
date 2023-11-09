#[cfg(any(feature="tests", test))]
use arbtest::arbitrary::{self};

#[derive(Debug)]
#[cfg_attr(any(feature="tests", test), derive(arbitrary::Arbitrary))]
pub enum AThing {
    A,
    B,
}