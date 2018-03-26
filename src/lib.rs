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
//! pager.set(0, )
//! ```

/// Memory pager instance. Manages [`Page`] instances.
///
/// [`Page`]: struct.Page.html
#[derive(Debug)]
pub struct Pager {
  /// The size of each page held in memory.
  pub page_size: usize,
  /// A vector of pages that are held in memory.
  pub pages: Vec<Page>,
  length: usize,
}

/// Memory pages returned by [`Pager`].
///
/// [`Pager`]: struct.Pager.html
#[derive(Debug)]
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

  /// Create a new [`Pager`] instance with a [`page_size`] and [`pages`]. Useful
  /// to restore a [`Pager`] instance from disk.
  ///
  /// [`Pager`]: struct.Pager.html
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  /// [`pages`]: struct.Pager.html#structfield.pages
  pub fn with_pages(page_size: usize, pages: Vec<Page>) -> Self {
    for page in &pages {
      assert_eq!(page.buffer.len(), page_size);
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
    if page_num >= self.pages.len() {
      self.grow_pages(page_num);
    }

    if let None = self.pages.get(page_num) {
      let mut buf: Vec<u8> = Vec::with_capacity(self.page_size);
      for _ in 0..self.page_size {
        buf.push(0);
      }
      let page = Page::new(page_num, buf);
      self.pages.insert(page_num, page)
    }

    self.pages.get(page_num).unwrap()
  }

  ///// Explicitely set a buffer for a [`Page`]. Useful to recreate pages that
  ///// have been persisted to disk.
  /////
  ///// [`Page`]: struct.Page.html
  //pub fn set(&mut self, index: usize, mut buf: Vec<u8>) {
  //  if index >= self.pages.len() {
  //    self.grow_pages(index);
  //  }

  //  if index >= self.length {
  //    self.length = index + 1;
  //  }

  //  self.resize_buffer(&mut buf);

  //  let page = self.pages.get_mut(index).unwrap();
  //  panic!("not finished implementing");

  //  // match  {
  //  //   Some(ref mut page) => page.buffer = buf,
  //  //   None => {
  //  //     let page = Page::new(index, buf);
  //  //     self.pages.insert(index, Some(page));
  //  //   }
  //  // }
  //}

  /// Grow the page buffer capacity to accomodate more elements.
  fn grow_pages(&mut self, index: usize) {
    let start_len = self.pages.len();
    let mut nlen = start_len * 2;
    while nlen <= index {
      nlen *= 2;
    }

    self.pages.reserve_exact(nlen);
  }

  ///// Resize a single buffer to fit the page size. Can both grow and shrink.
  /////
  ///// NOTE(yw): Unlike the original `memory-pager` implementation, we do not
  ///// zero out the memory, but instead rely on growing the capacity - and assume
  ///// the acquired memory is zeroed out. This feels more rust-like, but we have
  ///// to be careful that this won't lead to any bugs down the line.
  //fn resize_buffer(&mut self, buf: &mut Vec<u8>) {
  //  let buf_cap = buf.capacity();
  //  let len = self.page_size;
  //  if buf_cap > len {
  //    buf.truncate(len);
  //  } else if buf_cap < len {
  //    buf.reserve_exact(len);
  //  }
  //}
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
