# memory-pager
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Access memory using small fixed size buffers. Adapted from
[mafintosh/memory-pager](https://github.com/mafintosh/memory-pager).

- [Documentation][8]
- [Crate][2]

## Example
```rust
extern crate memory_pager;

let mut pager = memory_pager::Pager::new(1024);
let page = pager.get_mut_or_alloc(3);
assert_eq!(page.len(), 1024);
```

## Installation
```sh
$ cargo add memory-pager
```

## License
[Apache-2.0](./LICENSE)

[1]: https://img.shields.io/crates/v/memory-pager.svg?style=flat-square
[2]: https://crates.io/crates/memory-pager
[3]: https://img.shields.io/travis/datrs/memory-pager.svg?style=flat-square
[4]: https://travis-ci.org/datrs/memory-pager
[5]: https://img.shields.io/crates/d/memory-pager.svg?style=flat-square
[6]: https://crates.io/crates/memory-pager
[7]: https://docs.rs/memory-pager/badge.svg
[8]: https://docs.rs/memory-pager
