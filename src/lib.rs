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

  /// Create a new instance from a file.
  ///
  /// This is particularly useful when restoring the `memory-pager` from disk,
  /// as it's possible to open a file, and directly convert it into a pager
  /// instance.
  ///
  /// # Options
  ///
  /// The third argument is an optional `offset` of `usize`. This is useful to
  /// ignore the first few bytes if the file has a header that isn't part of the
  /// bitfield's body.
  ///
  /// # Errors
  ///
  /// This method will return an error if the `File` length is not a multiple of
  /// `page_size`. It can also fail if it's unable to read the file's metadata.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # extern crate memory_pager as pager;
  /// # extern crate failure;
  /// use failure::Error;
  /// use pager::Pager;
  /// use std::fs;
  ///
  /// fn main () -> Result<(), Error> {
  ///   let mut file = fs::File::open("tests/fixtures/40_empty.bin")?;
  ///   let page_size = 10;
  ///   let _pager = Pager::from_file(&mut file, page_size, None)?;
  ///   Ok(())
  /// }
  /// ```
  #[inline]
  pub fn from_file(
    file: &mut File,
    page_size: usize,
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
    let mut buf = vec![0; page_size];

    for index in 0..page_count {
      let bytes_read = file.read(&mut buf)?;

      // This should already be guarded for, but making extra extra sure.
      if bytes_read < page_size {
        break;
      }

      // The buffer is reused if it only contains zeroes.
      if is_zeroed(&buf) {
        pages.push(None);
      } else {
        pages.push(Some(Page::new(index, buf)));
        buf = vec![0; page_size];
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
    self.pages.get(page_num).and_then(|page| page.as_ref())
  }

  /// Get a mutable [`Page`] wrapped in an `Option` enum. Does not allocate on
  /// access.
  ///
  /// [`Page`]: struct.Page.html
  #[inline]
  pub fn get_mut(&mut self, page_num: usize) -> Option<&mut Page> {
    self.pages.get_mut(page_num).and_then(|page| page.as_mut())
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
