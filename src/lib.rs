#![deny(missing_docs)]
#![feature(external_doc)]
#![doc(include = "../README.md")]

// #![feature(generic_associated_types)]

mod page;
mod pager;

pub use page::*;
pub use pager::*;
