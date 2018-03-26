extern crate memory_pager;

use memory_pager::{Page, Pager};

#[test]
fn can_create_default() {
  let _pager = Pager::default();
}

#[test]
fn can_create_with_size() {
  let _pager = Pager::new(1024);
}

#[test]
fn can_get() {
  let mut pager = Pager::default();
  {
    let page = pager.get(0);
    assert_eq!(page.buffer.len(), 1024);
  }

  {
    let page = pager.get(3);
    assert_eq!(page.buffer.len(), 1024);
  }
}

#[test]
fn can_alloc() {
  let mut pager = Pager::default();
  {
    let page = &pager.get(16);
    assert_eq!(page.buffer.len(), 1024);
  }

  {
    let page = &pager.get(32);
    assert_eq!(page.buffer.len(), 1024);
  }
}
