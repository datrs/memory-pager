extern crate memory_pager;

use memory_pager::Pager;

// Postmortem:
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
