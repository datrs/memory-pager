#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate failure;

mod iter;
mod page;

pub use iter::Iter;
pub use page::Page;

use failure::Error;
use std::fs::File;
use std::io::Read;

/// Memory pager instance. Manages [`Page`] instances.
///
/// [`Page`]: struct.Page.html
#[derive(Debug)]
pub struct Pager {
  pages: Vec<Option<Page>>,
  page_size: usize,
}

impl Pager {
  /// Create a new [`Pager`] instance with a [`page_size`].
  ///
  /// [`Pager`]: struct.Pager.html
  /// [`page_size`]: struct.Pager.html#structfield.page_size
  #[inline]
  pub fn new(page_size: usize) -> Self {
    Pager {
      page_size,
      pages: Vec::new(),
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
  #[inline]
  pub fn from_pages(page_size: usize, pages: Vec<Option<Page>>) -> Self {
    for page in &pages {
      if let Some(ref page) = *page {
        assert_eq!(page.len(), page_size);
      }
    }

    Self { page_size, pages }
  }

  /// Create a new instance from a reader.
  ///
  /// This is particularly useful when restoring the `memory-pager` from disk,
  /// as it's possible to open a file, and directly convert it into a pager
  /// instance.
  ///
  /// ## Options
  /// The third argument is an optional offset of `usize`. This is useful to
  /// ignore the first few bytes if the file has a header that isn't part of the
  /// bitfield's body.
  ///
  /// ## Errors
  /// This method will fail if the `File` length is not a multiple of
  /// `page_size`.
  ///
  /// ## Example
  /// ```rust
  /// # extern crate memory_pager as pager;
  /// use pager::Pager;
  /// use std::fs;
  ///
  /// fn main () -> std::io::Result<()> {
  ///   let page_size = 1024;
  ///   let mut file = fs::File::open("file")?;
  ///   let _pager = Pager::from_reader(&file, page_size, None)?;
  /// }
  /// ```
  #[inline]
  pub fn from_file(
    page_size: usize,
    file: &mut File,
    offset: Option<usize>,
  ) -> Result<Self, Error> {
    let offset = offset.unwrap_or(0);
    let len = file.metadata()?.len() as usize - offset;

    ensure!(
      len % page_size == 0,
      format!(
        "<memory-pager>: Reader len ({}) is not a multiple of {}",
        len, page_size
      )
    );

    let page_count = len / page_size;
    let mut pages = Vec::with_capacity(page_count);
    let mut buf = Vec::with_capacity(page_size);

    for index in 0..page_count {
      file.read_exact(&mut buf)?;

      // The buffer is reused if it only contains zeroes.
      if is_zeroed(&buf) {
        pages.push(None);
      } else {
        pages.push(Some(Page::new(index, buf)));
        buf = Vec::with_capacity(page_size);
      }
    }

    Ok(Self { pages, page_size })
  }

  /// Get a [`Page`] mutably. The page will be allocated on first access.
  ///
  /// [`Page`]: struct.Page.html
  #[inline]
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

    self.pages[page_num].as_mut().unwrap()
  }

  /// Get a [`Page`] wrapped in an `Option` enum. Does not allocate on access.
  ///
  /// [`Page`]: struct.Page.html
  #[inline]
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
  #[inline]
  pub fn get_mut(&mut self, page_num: usize) -> Option<&mut Page> {
    match self.pages.get_mut(page_num) {
      None => None,
      Some(page) => page.as_mut(),
    }
  }

  /// Grow the page buffer capacity to accommodate more elements.
  #[inline]
  fn grow_pages(&mut self, index: usize) {
    self.pages.resize(index + 1, None);
  }

  /// The number of pages held by `memory-pager`. Doesn't account for empty
  /// entries. Comparable to `vec.len()` in usage.
  #[inline]
  pub fn len(&self) -> usize {
    self.pages.len()
  }

  /// check whether the `length` is zero.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.pages.is_empty()
  }

  /// Get the memory page size in bytes.
  #[inline]
  pub fn page_size(&self) -> usize {
    self.page_size
  }

  /// Iterate over `&Pages`.
  #[inline]
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

#[inline]
fn is_zeroed(vec: &[u8]) -> bool {
  for byte in vec {
    if *byte != 0 {
      return false;
    }
  }
  true
}
