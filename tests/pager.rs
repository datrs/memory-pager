mod pager {
  extern crate memory_pager;

  #[test]
  fn can_create_default() {
    let _pager = memory_pager::default();
  }

  #[test]
  fn can_create_with_size() {
    let _pager = memory_pager::new(1024);
  }
}
