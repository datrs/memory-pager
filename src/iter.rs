use super::{Page, Pager};
use std::{iter, slice};

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
  inner: slice::Iter<'a, Option<Page>>,
}

impl<'a> Iter<'a> {
  #[inline]
  pub(crate) fn new(pager: &'a Pager) -> Self {
    Self {
      inner: pager.pages.iter(),
    }
  }
}

impl<'a> iter::Iterator for Iter<'a> {
  type Item = &'a Option<Page>;

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}
