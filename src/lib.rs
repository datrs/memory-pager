#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

mod iter;
mod page;

pub use iter::Iter;
pub use page::Page;

/// Memory pager instance. Manages [`Page`] instances.
///
/// [`Page`]: struct.Page.html
#[derive(Debug)]
pub struct Pager {
  pages: Vec<Option<Page>>,
  length: usize,
  page_size: usize,
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
      pages: vec![None; 16],
    }
  }

  /// Create a new [`Pager`] instance with a [`page_size`] and [`pages`]. Useful
  /// to restore a [`Pager`] instance from disk.
  ///
  /// ## Panics
  /// Each page that's passed in `pages` must be the same length as `page size`.
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
      length: pages.len(),
      page_size,
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
  pub fn get(&self, page_num: usize) -> Option<&Page> {
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

  /// Grow the page buffer capacity to accommodate more elements.
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

  /// Get the memory page size in bytes.
  #[inline]
  pub fn page_size(&self) -> usize {
    self.page_size
  }

  /// Iterate over `&Pages`.
  pub fn iter(&self) -> Iter {
    Iter::new(self)
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
