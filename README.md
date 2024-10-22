# Linkcheck

[![Continuous integration](https://github.com/marxin/linkcheck2/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/marxin/linkcheck2/actions)
[![Docs.rs Badge](https://docs.rs/linkcheck2/badge.svg)](https://docs.rs/linkcheck2)
[![Crates.io](https://img.shields.io/crates/v/linkcheck2)](https://crates.io/crates/linkcheck2)
![Crates.io](https://img.shields.io/crates/l/linkcheck2)

A library for extracting and validating links.

For insight into how this crate is designed and was implemented, you may want to
read [the original blog post][blog].

This is a **fork** of the [linkcheck](https://github.com/Michael-F-Bryan/linkcheck) crate.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[crev]: https://github.com/crev-dev/cargo-crev
[blog]: http://adventures.michaelfbryan.com/posts/linkchecker/
