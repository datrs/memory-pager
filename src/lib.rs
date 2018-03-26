#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! Access memory using small fixed size buffers. Adapted from
//! [mafintosh/memory-pager](https://github.com/mafintosh/memory-pager).
//!
//! ```rust,ignore
//! extern crate memory_pager;
//!
//! let pager = memory_pager::Pager::new(1024);
//! ```

/// Memory pager.
pub struct Pager {
  /// Size in bytes of each memory page. Default page size is `1024`.
  page_size: usize,
}

impl Pager {
  /// Create a new `Pager` instance with a page size.
  pub fn new(page_size: usize) -> Self {
    Pager {
      page_size: page_size,
    }
  }

  /// Get a page. The page will be allocated on first access.
  pub fn get(&mut self) {}

  /// Explicitely set a buffer for a page.
  pub fn set(&mut self) {}

  /// Get the last page that was updated.
  pub fn last_update(&mut self) {}
}

impl Default for Pager {
  fn default() -> Self {
    Pager::new(1024)
  }
}
