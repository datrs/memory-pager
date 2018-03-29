extern crate memory_pager;

use memory_pager::Pager;

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
    let page = pager.get_mut_or_alloc(0);
    assert_eq!(page.len(), 1024);
  }

  {
    let page = pager.get_mut_or_alloc(3);
    assert_eq!(page.len(), 1024);
  }
}

#[test]
fn can_alloc() {
  let mut pager = Pager::default();
  {
    let page = &pager.get_mut_or_alloc(16);
    assert_eq!(page.len(), 1024);
  }

  {
    let page = &pager.get_mut_or_alloc(32);
    assert_eq!(page.len(), 1024);
  }
}

#[test]
fn can_write() {
  let mut pager = Pager::default();
  let page = pager.get_mut_or_alloc(0);
  page[0] = 1;

  assert_eq!(1, page[0]);
  assert_eq!(0, page[1]);
}

#[test]
fn can_check_offset() {
  let mut pager = Pager::default();
  let page = pager.get_mut_or_alloc(1);

  assert_eq!(1024, page.offset());
}

#[test]
fn can_access_nodes() {
  let pager = &mut Pager::default();
  assert!(pager.get(0).is_none());
  pager.get_mut_or_alloc(0);
  assert!(pager.get(0).is_some());
}
