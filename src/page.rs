use std::ops::{Deref, DerefMut};

/// Memory pages returned by [`Pager`].
///
/// [`Pager`]: struct.Pager.html
#[derive(Debug, Clone)]
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
  pub(crate) fn new(i: usize, buf: Vec<u8>) -> Self {
    Page {
      offset: i * buf.capacity(),
      buffer: buf,
    }
  }

  /// Byte offset for the start of the `Page` relative to all other `Page`
  /// instances.
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
