# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

## [1.3.0] - 2019-12-29

### Added
- Methods `MinMaxHeap::peek_min_mut` and `MinMaxHeap::peek_max_mut`*.

### Changed
- Oldest supported rustc version is now 1.32.0.
- Updated dev dependencies `rand` (0.5 to 0.7) and `quickcheck` (0.7.2
  to 0.9).

## [1.2.2] - 2018-12-17

### Fixed
- Removed syntax not accepted by older rustc.

## [1.2.1] - 2018-12-17

### Fixed
- Documented time complexity of `push` to be worst-case linear and amortized 
  logarithmic.
- Fixed bugs in `MinMaxHeap::replace_max` and `MinMaxHeap::push_pop_max` that 
  were corrupting the heap order.

## [1.2.0] - 2018-06-02

### Added
- Two new iterator types, `DrainAsc` and `DrainDesc`, and two new methods
  for creating them, `MinMaxHeap::drain_asc` and `MinMaxHeap::drain_desc`.
  These iterators drain the heap in ascending (min-first) or descending
  (max-first) order, respectively.
- Implementations of `Iterator::size_hint` for `Drain`, `IntoIter`, and 
  `Iter` iterator types.
  
## [1.1.1] - 2018-05-30

### Added
- `#[doc(html_base_url = ...)]` annotation in crate root.

## [1.1.0] - 2018-05-12

### Added
- Optional serde support. Enable `"serde"` Cargo feature to get impls
  of `Serializable` and `Deserializable` for `MinMaxHeap`.
- Documented oldest supported rustc version: 1.20.

## [1.0.4] - 2018-05-04

### Changed
- Uses `ManuallyDrop` instead of `Option` in internal `Hole`
  implementation. (Thanks to Nikita Popov.)
  
## [1.0.3] - 2018-05-03

### Added
- Some simple benchmarks.

### Changed
- Internal `is_min_level` function uses `leading_zeros` instead of an
  *O*(log *n*)–time loop.
  
## [1.0.2] - 2018-04-01

### Fixed
- Documentation URL.

## [1.0.1] - 2018-03-31

### Removed
- Dependency on Clippy. Use `cargo +nightly clippy` instead.

## [1.0.0] - 2018-03-31

### Added
- Automatic code coverage checking on codecov.io.

## [0.2.1] - 2016-06-13

### Fixed
- Bad crate metadata.

## [0.2.0] - 2016-06-13

### Added
- `MinMaxHeap::into_vec_asc` and `MinMaxHeap::into_vec_desc` methods.
- Impl of `From<Vec<T>>` for `MinMaxHeap<T>`.

## [0.1.2] - 2016-06-13

### Removed
- Clippy warnings.

## [0.1.1] - 2016-06-12

### Added
- Mentioned `crates.io` in docs.

## [0.1.0] - 2016-06-12

Initial release

  
