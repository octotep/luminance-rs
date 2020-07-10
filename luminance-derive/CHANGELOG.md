# Changelog

This document is the changelog of [luminance-derive](https://crates.io/crates/luminance-derive).
You should consult it when upgrading to a new version, as it contains precious information on
breaking changes, minor additions and patch notes.

**If you’re experiencing weird type errors when upgrading to a new version**, it might be due to
how `cargo` resolve dependencies. `cargo update` is not enough, because all luminance crate use
[SemVer ranges](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) to stay
compatible with as many crates as possible. In that case, you want `cargo update --aggressive`.

<!-- vim-markdown-toc GFM -->

* [0.6](#06)
* [0.5.2](#052)
* [0.5.1](#051)
* [0.5](#05)
* [0.4](#04)
  * [Major changes](#major-changes)
  * [Minor changes](#minor-changes)
  * [Patch changes](#patch-changes)
* [0.3](#03)
* [0.2.2](#022)
* [0.2.1](#021)
* [0.2](#02)
* [0.1.1](#011)
* [0.1](#01)

<!-- vim-markdown-toc -->

# 0.6

> ?

- The `UniformInterface` proc-macro got patched to adapt to the new backend architecture.
- Implement `std::error::Error` for various types of the crate.
- Add helper methods to create error types requiring owned data via a better API.

# 0.5.2

> Tue Jan 7th 2020

- Add `Deref` and `DerefMut` implementors for semantics’ generated variant types. You can now
  access the underlying (wrapped) repr type.
- In the case of `Deref` and `DerefMut` not being enough, the underlying field can also be
  directly accessed (it’s now `pub`).

# 0.5.1

> Sat Jan 4th 2020

- Support of `luminance-0.38`.

# 0.5

> Sun Sep 29th 2019

- Support of `luminance-0.37`.

# 0.4

> Fri Sep 20th 2019

## Major changes

- Add `new` methods for types annotated with `Vertex`. This is considered a breaking change as
  it would break your code if you already have a `new` method, which is very likely.

## Minor changes

- Add support for struct-tuple when deriving `Vertex`.

## Patch changes

- Empty `Semantics` types are forbidden and now reported correctly as errors.

# 0.3

> Thur Sep 12th 2019

- Fix SemVer issues with ranges and duplicated dependencies.

# 0.2.2

> Thur Sep 12th 2019

- Support of `luminance-0.35`.

# 0.2.1

> Wed Sep 11th 2019

- Support of `luminance-0.34`.

# 0.2

> Fri Sep 6th 2019

- Support of `luminance-0.33`.

# 0.1.1

> Tue Sep 3rd 2019

- Support of `luminance-0.32`.

# 0.1

> Fri Aug 23th 2019

- Initial revision.
