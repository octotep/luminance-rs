# Changelog

This document is the changelog of [luminance-glfw](https://crates.io/crates/luminance-glfw).
You should consult it when upgrading to a new version, as it contains precious information on
breaking changes, minor additions and patch notes.

**If you’re experiencing weird type errors when upgrading to a new version**, it might be due to
how `cargo` resolve dependencies. `cargo update` is not enough, because all luminance crate use
[SemVer ranges](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) to stay
compatible with as many crates as possible. In that case, you want `cargo update --aggressive`.

<!-- vim-markdown-toc GFM -->

* [0.12](#012)
* [0.11](#011)
* [0.10](#010)
  * [Minor changes](#minor-changes)
* [0.9](#09)
* [0.8.1](#081)
* [0.8](#08)
* [0.7](#07)
* [0.6.1](#061)
* [0.6](#06)
  * [Major changes](#major-changes)
  * [Minor changes](#minor-changes-1)
  * [Patch & misc changes](#patch--misc-changes)
* [0.5.4](#054)
* [0.5.3](#053)
* [0.5.2](#052)
* [0.5.1](#051)
* [0.5](#05)
* [0.4.3](#043)
* [0.4.2](#042)
* [0.4.1](#041)
* [0.4](#04)
* [0.3.3](#033)
* [0.3.2](#032)
* [0.3.1](#031)
* [0.3](#03)
* [0.2](#02)
* [0.1.5](#015)
* [0.1.4](#014)
* [0.1.3](#013)
* [0.1.2](#012-1)
* [0.1.1](#011-1)
* [0.1](#01)

# 0.13

>

- Support of `luminance-0.40`.
- Support of `glfw-0.39`.
- Add helper methods to create error types requiring owned data via a better API.
- Remove the `WindowDim` argument in `new_gl33`. You can pass that argument via the `WindowOpt`
  dimension option.

# 0.12.2

> Wed Apr 22nd 2020

- Make the `glfw` fields in `GlfwSurface` `pub` to allow people customizing them further.

# 0.12.1

> Sat Feb 29th 2020

- Support of `luminance-0.39`.

# 0.12

> Sat Jan 4th 2020

- Support of `luminance-0.38`.
- Support of `glfw-0.34`.

# 0.11

> Sun Sep 29th 2019

- Support of `luminance-0.37`.

# 0.10

> Fri Sep 20th 2019

## Minor changes

- Add the `log-errors` feature-flags, allowing not to fail on GLFW errors but instead log them.

# 0.9

> Thur Sep 12th 2019

- Fix SemVer issues with ranges and duplicated dependencies.

# 0.8.1

> Thur Sep 12th 2019

- Support of `luminance-0.35`.

# 0.8

> Wed Sep 11th 2019

- Support of `luminance-0.34`.

# 0.7

> Fri Sep 6th 2019

- Support of `luminance-0.33`.

# 0.6.1

> Tue Sep 3rd 2019

- Support of `luminance-0.32`.

# 0.6

> Fri Aug 23th 2019

## Major changes

- Move `swap_buffers` from `GraphicsContext` to `Surface` in [luminance-windowing].

## Minor changes

- The `WindowOpt` now has support for multisampling. See the `WindowOpt::set_num_samples` for
  further details.
- Migrate to Rust Edition 2018.
- Implement dynamic edition of windowing types properties. That allows to change data on-the-fly,
  such as the cursor mode.

## Patch & misc changes

- Add more CI testing.
- Massive documentation rewrite (among the use of `#![deny(missing_docs)]`. The situation is still
  not perfect and patch versions will be released to fix and update the documentation. Step by
  step.
- Massive dependencies update. Special thanks to @eijebong for his help!

# 0.5.4

> Thursday, 29th of July, 2018

- Add support for `luminance-0.30`.

# 0.5.3

> Tuesday, 13th of July, 2018

- Add support for `luminance-0.29`.

# 0.5.2

> Tuesday, 3rd of July, 2018

- Add support for `luminance-0.28`.

# 0.5.1

> Friday, 29th of June, 2018

- Add support for `luminance-0.27`.

# 0.5

> Monday, 18th of June, 2018

- Implement the `luminance` backend interface.
- Support for the new `luminance-windowing` system.
- Remove the concept of `Device` and introduce the concept of `Surface`.

# 0.4.3

> Tuesday, 13th of February, 2018

- Support for `gl-0.10`.

# 0.4.2

> Sunday, 28th of January, 2018

- Support for `gl-0.9`.
- Support for `glfw-0.20`.

# 0.4.1

> Monday, 2nd of October, 2017

- Implement `Display` and `Error` for `GLFWDeviceError`.

# 0.4

> Sunday, 1st of October, 2017

- Use `luminance-windowing` to benefit from the types and functions defined in there.

# 0.3.3

> Saturday, 30th of September, 2017

- Remove the `luminance` dependency as it’s not needed anymore.

# 0.3.2

- All events are now polled, thus all kinds of events are now inspectable.

# 0.3.1

- Support for `glfw-0.16`.

# 0.3

- Removed `open_window` and moved its code into `Device::new`.
- Enhanced the documentation.
- Implemented Hi-DPI (tested on a Macbook Pro).
- Removed `Device::width` and `Device::height` and replaced them with `Device::size`.

# 0.2

- Changed the way events are handled. It doesn’t create an events thread anymore but instead exposes
  a polling interface. It’ll enhance performance and make mono-thread systems work, like Mac OSX.

# 0.1.5

- Internal fix for Mac OSX.

# 0.1.4

- Updated `luminance` dependencies.

# 0.1.3

- Changed the trait bound from `Fn` to `FnOnce` on `Device::draw`.

# 0.1.2

- Made `glfw::InitError` visible.

# 0.1.1

- `Device::draw` added.

# 0.1

- Initial revision.
