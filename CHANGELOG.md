## 2019-09-07, Version 0.9.0
### Commits
- [[`5b4623a7f3`](https://github.com/datrs/memory-pager/commit/5b4623a7f3f79af9a2a217d53139541dd6a885f3)] (cargo-release) version 0.9.0 (Yoshua Wuyts)
- [[`c63a9a46df`](https://github.com/datrs/memory-pager/commit/c63a9a46df0649fc9458e6bea9bfe93eee32203a)] update err bound (Yoshua Wuyts)
- [[`d46cac6d02`](https://github.com/datrs/memory-pager/commit/d46cac6d02f6b8661b1826b768a9bf4cab5d1b39)] Merge pull request #20 from bltavares/maintenence (Szabolcs Berecz)
- [[`c328349982`](https://github.com/datrs/memory-pager/commit/c328349982e2dbf683043c3887be0c2e08f6093f)] Bump code to Rust 2018 edition (Bruno Tavares)
- [[`8f3f650af4`](https://github.com/datrs/memory-pager/commit/8f3f650af47a0044e56b48d6aeb1037a1a0555c5)] Update rand requirement from 0.5.5 to 0.6.0 (dependabot[bot])
- [[`ec41dc3123`](https://github.com/datrs/memory-pager/commit/ec41dc312376cb93984ee66c5d88e18d5a9c5248)] Update quickcheck requirement from 0.6.2 to 0.7.1 (#12) (dependabot[bot])
- [[`a9283d0f8c`](https://github.com/datrs/memory-pager/commit/a9283d0f8c09dbdd77bbce99ae58ce1b685ab871)] Keep up with modern times in clippy invocation (#11) (Szabolcs Berecz)
- [[`27b43742e1`](https://github.com/datrs/memory-pager/commit/27b43742e161c5798316bbdfbd130b7f51906355)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml         |  4 ++--
 CHANGELOG.md        | 21 +++++++++++++++++++++
 Cargo.toml          |  8 ++++----
 src/lib.rs          | 29 ++++++++++++++---------------
 tests/model.rs      |  9 +++------
 tests/regression.rs |  2 --
 tests/test.rs       |  7 ++-----
 7 files changed, 46 insertions(+), 34 deletions(-)
```


## 2018-08-31, Version 0.8.0
### Commits
- [[`544d352173`](https://github.com/datrs/memory-pager/commit/544d352173ad55e417b4a3db6b1543ee8f00ab5d)] (cargo-release) version 0.8.0 (Yoshua Wuyts)
- [[`4862159cbf`](https://github.com/datrs/memory-pager/commit/4862159cbff71e5183af87b2d2fec13d79a88f91)] read file to buf not only once (#10) (周汉成)
- [[`907cf16695`](https://github.com/datrs/memory-pager/commit/907cf16695fbba2b4a472c72e484d41fd2bcff41)] Update .github (Yoshua Wuyts)
- [[`58458af8fa`](https://github.com/datrs/memory-pager/commit/58458af8fa3fce2f413539ee90b9830942baa8ce)] (cargo-release) start next development iteration 0.7.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 .github/ISSUE_TEMPLATE.md                 | 40 +++-----------------------------
 .github/ISSUE_TEMPLATE/bug_report.md      | 23 ++++++++++++++++++-
 .github/ISSUE_TEMPLATE/feature_request.md | 30 ++++++++++++++++++++++++-
 .github/ISSUE_TEMPLATE/question.md        | 18 ++++++++++++++-
 Cargo.toml                                |  2 +-
 src/lib.rs                                |  2 +-
 tests/fixtures/40_normal.txt              |  1 +-
 tests/test.rs                             |  9 ++++++-
 8 files changed, 86 insertions(+), 39 deletions(-)
```


