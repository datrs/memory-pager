mod pager {
  extern crate memory_pager;

  #[test]
  fn can_create_default() {
    let _pager = memory_pager::Pager::default();
  }

  #[test]
  fn can_create_with_size() {
    let _pager = memory_pager::Pager::new(1024);
  }
}
