extern crate memory_pager;

use memory_pager::Pager;

// Postmortem: we were growing pages based on capacity, not on length. This
// means we could be accessing allocated, but uninitialized memory - which
// causes failures to occur.
#[test]
fn regression_1() {
  let mut pager = Pager::new(10);
  {
    pager.get_mut_or_alloc(8);
  }
  {
    pager.get_mut_or_alloc(24);
  }
}

// Postmortem: we were using `.insert()` inside `get_mut_or_alloc` which shifts
// values one to the right. This caused the value at postition 24 to become the
// value at position 25 after we did an insert before it. We fixed it by just
// overriding the value.
#[test]
fn regression_2() {
  let mut pager = Pager::new(10);
  {
    pager.get_mut_or_alloc(24);
  }
  {
    pager.get_mut_or_alloc(18);
  }
  {
    assert!(pager.get(25).is_none());
  }
}
