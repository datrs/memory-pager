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
//! let page = pager.get(3);
//! assert_eq!(page.len(), 1024);
//! ```

use std::ops::{Deref, DerefMut};

/// Memory pager instance. Manages [`Page`] instances.
///
/// [`Page`]: struct.Page.html
#[derive(Debug)]
pub struct Pager {
  /// The size of each page held in memory.
  pub page_size: usize,
  /// A vector of pages that are held in memory.
  pub pages: Vec<Option<Page>>,
  length: usize,
}

/// Memory pages returned by [`Pager`].
///
/// [`Pager`]: struct.Pager.html
#[derive(Debug)]
pub struct Page {
  /// Byte offset for the start of the `Page` relative to all other `Page`
  /// instances.
  offset: usize,
  /// Buffer with capacity of size [`page_size`].
  ///
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  buffer: Vec<u8>,
}

impl Page {
  fn new(i: usize, buf: Vec<u8>) -> Self {
    Page {
      offset: i * buf.capacity(),
      buffer: buf,
    }
  }

  pub fn offset(&self) -> usize {
      self.offset
  }
}

impl Deref for Page {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for Page {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl Pager {
  /// Create a new [`Pager`] instance with a [`page_size`].
  ///
  /// [`Pager`]: struct.Pager.html
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  pub fn new(page_size: usize) -> Self {
    let mut pages = Vec::with_capacity(16);
    for _ in 0..16 {
      pages.push(None);
    }
    Pager {
      page_size: page_size,
      length: 0,
      pages: pages,
    }
  }

  /// Create a new [`Pager`] instance with a [`page_size`] and [`pages`]. Useful
  /// to restore a [`Pager`] instance from disk.
  ///
  /// [`Pager`]: struct.Pager.html
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  /// [`pages`]: struct.Pager.html#structfield.pages
  pub fn with_pages(page_size: usize, pages: Vec<Option<Page>>) -> Self {
    for page in &pages {
      if let &Some(ref page) = page {
        assert_eq!(page.len(), page_size);
      }
    }

    Pager {
      page_size: page_size,
      length: pages.len(),
      pages: pages,
    }
  }

  /// Get a [`Page`]. The page will be allocated on first access.
  ///
  /// [`Page`]: struct.Page.html
  pub fn get(&mut self, page_num: usize) -> &Page {
    if page_num >= self.pages.capacity() {
      self.grow_pages(page_num);
    }

    // This should never be out of bounds.
    if let None = self.pages[page_num] {
      let mut buf = vec![0; self.page_size];
      let page = Page::new(page_num, buf);
      self.pages.insert(page_num, Some(page));
    }

    self.pages[page_num].as_ref().unwrap()
  }

  /// Get a [`Page`]. The page will be allocated on first access.
  ///
  /// [`Page`]: struct.Page.html
  pub fn get_mut(&mut self, page_num: usize) -> &mut Page {
    if page_num >= self.pages.capacity() {
      self.grow_pages(page_num);
    }

    // This should never be out of bounds.
    if let None = self.pages[page_num] {
      let mut buf: Vec<u8> = Vec::with_capacity(self.page_size);
      for _ in 0..self.page_size {
        buf.push(0);
      }
      let page = Page::new(page_num, buf);
      self.pages.insert(page_num, Some(page));
    }

    self.pages[page_num].as_mut().unwrap()
  }

  /// Grow the page buffer capacity to accomodate more elements.
  fn grow_pages(&mut self, index: usize) {
    let start_len = self.pages.capacity();
    let mut nlen = start_len * 2;

    // Guard against a page size of 0.
    if nlen == 0 {
      nlen += 1
    }

    while nlen <= index {
      nlen *= 2;
    }

    self.pages.reserve_exact(nlen);

    for _ in start_len..nlen {
      self.pages.push(None);
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
