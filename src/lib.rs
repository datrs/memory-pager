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
  /// Byte offset for the start of the `Page` relative to all other `Page`
  /// instances.
  pub offset: usize,
  /// Buffer with capacity of size [`page_size`].
  ///
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  pub buffer: Vec<u8>,
}

impl Page {
  fn new(i: usize, buf: Vec<u8>) -> Self {
    Page {
      offset: i * buf.capacity(),
      buffer: buf,
    }
  }
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
  pub fn get(&mut self) -> &Page {}

  /// Explicitely set a buffer for a [`Page`].
  ///
  /// [`Page`]: struct.Page.html
  pub fn set(&mut self, index: usize, buf: Vec<u8>) {
    if index >= self.pages.len() {
      self.grow_pages(index);
    }

    if index >= self.length {
      self.length = index + 1;
    }

    self.resize_buffer(&buf);

    match self.pages.get(index) {
      Some(page) => page.buffer = buf,
      None => self.pages[index] = Page::new(index, buf),
    }
  }

  /// Grow the page buffer capacity to accomodate more elements.
  fn grow_pages(&mut self, index: usize) {
    let list = self.pages;
    let len = self.length;

    let mut nlen = list.len() * 2;
    while nlen <= index {
      nlen *= 2;
    }

    self.pages.reserve_exact(nlen);
  }

  /// Resize a single buffer to fit the page size. Can both grow and shrink.
  ///
  /// NOTE(yw): Unlike the original `memory-pager` implementation, we do not
  /// zero out the memory, but instead rely on growing the capacity - and assume
  /// the acquired memory is zeroed out. This feels more rust-like, but we have
  /// to be careful that this won't lead to any bugs down the line.
  fn resize_buffer(&mut self, buf: &Vec<u8>) {
    let buf_cap = buf.capacity();
    let len = self.page_size;
    if buf_cap > len {
      buf.truncate(len);
    } else if buf_cap < len {
      buf.reserve_exact(len);
    }
  }
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
