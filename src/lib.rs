// #![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Access memory using small fixed size buffers. Adapted from
//! [mafintosh/memory-pager](https://github.com/mafintosh/memory-pager).
//!
//! ```rust,ignore
//! extern crate memory_pager;
//!
//! let pager = memory_pager::Pager::new(1024);
//! ```

/// Memory pager instance. Manages [`Page`] instances.
///
/// [`Page`]: struct.Page.html
pub struct Pager {
  /// The size of each page held in memory.
  pub page_size: usize,
  pages: Vec<Page>,
  length: usize,
}

/// Memory pages returned by [`Pager`].
///
/// [`Pager`]: struct.Pager.html
pub struct Page {
  /// Byte offset for the start of the `Page` relative to all other `Page`s.
  pub offset: usize,
  /// Buffer of size [`page_size`].
  ///
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  pub buffer: Vec<u8>,
}

impl Pager {
  /// Create a new [`Pager`] instance with a [`page_size`].
  ///
  /// [`Pager`]: struct.Pager.html
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  pub fn new(page_size: usize) -> Self {
    Pager {
      page_size: page_size,
      length: 0,
      pages: Vec::with_capacity(1),
    }
  }

  /// Get a [`Page`]. The page will be allocated on first access.
  ///
  /// [`Page`]: struct.Page.html
  pub fn get(&mut self) {}

  /// Explicitely set a buffer for a [`Page`].
  ///
  /// [`Page`]: struct.Page.html
  pub fn set(&mut self, index: usize, buf: Vec<u8>) {}
}

/// Create a new [`Pager`] instance with a [`page_size`] of `1024`.
///
/// [`Pager`]: struct.Pager.html
/// [`page_size`]: struct.Pager.html#structfield.page_size
impl Default for Pager {
  fn default() -> Self {
    Pager::new(1024)
  }
}
