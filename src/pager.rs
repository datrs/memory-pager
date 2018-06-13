use super::*;
use std::{borrow, iter};

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
  /// Position in the iterator.
  cursor: usize,
}

impl Pager {
  /// Create a new [`Pager`] instance with a [`page_size`].
  ///
  /// [`Pager`]: struct.Pager.html
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  pub fn new(page_size: usize) -> Self {
    Pager {
      page_size,
      length: 0,
      cursor: 0,
      pages: vec![None; 16],
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
      if let Some(ref page) = *page {
        assert_eq!(page.len(), page_size);
      }
    }

    Pager {
      cursor: 0,
      page_size,
      length: pages.len(),
      pages,
    }
  }

  /// Get a [`Page`] mutably. The page will be allocated on first access.
  ///
  /// [`Page`]: struct.Page.html
  pub fn get_mut_or_alloc(&mut self, page_num: usize) -> &mut Page {
    if page_num >= self.pages.len() {
      self.grow_pages(page_num);
    }

    // This should never be out of bounds.
    if self.pages[page_num].is_none() {
      let buf = vec![0; self.page_size];
      let page = Page::new(page_num, buf);
      self.pages[page_num] = Some(page);
    }

    if page_num > self.length {
      self.length = page_num + 1;
    }

    self.pages[page_num].as_mut().unwrap()
  }

  /// Get a [`Page`] wrapped in an `Option` enum. Does not allocate on access.
  ///
  /// [`Page`]: struct.Page.html
  pub fn get(&mut self, page_num: usize) -> Option<&Page> {
    match self.pages.get(page_num) {
      None => None,
      Some(page) => page.as_ref(),
    }
  }

  /// Get a mutable [`Page`] wrapped in an `Option` enum. Does not allocate on
  /// access.
  ///
  /// [`Page`]: struct.Page.html
  pub fn get_mut(&mut self, page_num: usize) -> Option<&mut Page> {
    match self.pages.get_mut(page_num) {
      None => None,
      Some(page) => page.as_mut(),
    }
  }

  /// Grow the page buffer capacity to accomodate more elements.
  fn grow_pages(&mut self, index: usize) {
    let start_len = self.pages.len();
    let mut new_len = start_len * 2;

    // Guard against a page size of 0.
    if new_len == 0 {
      new_len += 1
    }

    while new_len <= index {
      new_len *= 2;
    }

    self.pages.resize(new_len, None);
  }

  /// The number of pages held by `memory-pager`. Doesn't account for empty
  /// entries. Comparable to `vec.len()` in usage.
  #[inline]
  pub fn len(&self) -> usize {
    self.length
  }

  /// check whether the `length` is zero.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
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

// TODO: return a new struct that holds a lifetime so we can iterate over it.
// The API should probably become `.iter()` which returns an iterator.
impl<'a> iter::Iterator for Pager {
  type Item = &'a Option<Page>;

  fn next(&mut self) -> Option<&Self::Item> {
    let cursor = self.cursor;
    self.cursor += 1;

    if cursor >= self.length {
      None
    } else {
      self.pages.get(cursor)
    }
  }

  // fn size_hint(&self) -> (usize, Option<usize>) {
  //   (0, Some(self.len()))
  // }
}
