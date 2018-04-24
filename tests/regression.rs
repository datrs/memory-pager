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
