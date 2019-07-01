# rust debug formatting enhancer

Simple tool that reformats some subset of Rust's Debug style formatting (`{:?}`)
to be easier readable (like `{:#?}`). Useful when you can't change the output
yourself.

[![Travis Status](https://travis-ci.org/killercup/rust-dbg-enhancer.svg?branch=master)](https://travis-ci.org/killercup/rust-dbg-enhancer)

## Install

`cargo install --git https://github.com/killercup/rust-dbg-enhancer`

## Usage

Simple:

```console
$ echo 'Pair { rule: group, span: Span { str: "[Level2]", start: 204, end: 212 }, inner: [] }' | rust-dbg-enhancer
Pair {
    rule: group,
    span: Span {
        str: "[Level2]'",
        start: 204,
        end: 212,
    },
    inner: [],
}
```

Using macOS copy-paste helpers:

```console
$ pbpaste | rust-dbg-enhancer
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

