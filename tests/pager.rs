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

  #[test]
  fn can_set() {
    let pager = memory_pager::Pager::new(1024);
    let buf: Vec<u8> = [b"hi", b"hello"];
    pager.set(0, buf);
  }
}
