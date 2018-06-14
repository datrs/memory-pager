use std::iter;
use super::{Page, Pager};

/// Iterator over a `Pager` instance.
///
/// ```rust
/// # extern crate memory_pager;
/// # use memory_pager::Pager;
/// let mut pager = Pager::default();
/// pager.get_mut_or_alloc(1);
/// pager.get_mut_or_alloc(2);
/// pager.get_mut_or_alloc(3);
///
/// for page in pager.iter() {
///   println!("page {:?}", page);
/// }
/// ```
pub struct Iter<'a> {
  inner: &'a Pager,
  cursor: usize,
}

impl<'a> Iter<'a> {
  #[inline]
  pub(crate) fn new(pager: &'a Pager) -> Self {
    Self {
      inner: pager,
      cursor: 0,
    }
  }
}

impl<'a> iter::Iterator for Iter<'a> {
  type Item = &'a Option<Page>;

  fn next(&mut self) -> Option<Self::Item> {
    let cursor = self.cursor;
    self.cursor += 1;

    if cursor >= self.inner.len() {
      None
    } else {
      self.inner.pages.get(cursor)
    }
  }
}
