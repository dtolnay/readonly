Readonly
========

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/readonly-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/readonly)
[<img alt="crates.io" src="https://img.shields.io/crates/v/readonly.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/readonly)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-readonly-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/readonly)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/readonly/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/readonly/actions?query=branch%3Amaster)

This crate provides an attribute macro to expose struct fields that are readable
and writable from within the same module but readable only outside the module.

```toml
[dependencies]
readonly = "0.2"
```

## Syntax

Place `#[readonly::make]` on a braced struct or tuple struct. This will make all
fields of the struct publicly readable according to their individual visibility
specifiers, but not writable from other modules.

```rust
mod m {
    #[readonly::make]
    pub struct S {
        pub n: i32,
    }

    impl S {
        pub fn demo(&mut self) {
            // Can read and write from inside the same module.
            println!("{}", self.n);
            self.n += 1;
        }
    }
}

fn demo(s: &mut m::S) {
    // From outside the module, can only read.
    println!("{}", s.n);

    // Does not compile:
    //s.n += 1;
}
```

The error appears as follows.

```console
error[E0594]: cannot assign to data in a dereference of `m::S`
  --> readme.rs:21:5
   |
21 |     s.n += 1;
   |     ^^^^^^^^ cannot assign
```

Optionally, place `#[readonly]` on individual struct fields to make just those
fields publicly readable, without affecting other fields of the struct.

```rust
#[readonly::make]
pub struct S {
    // This field can be read (but not written) by super.
    #[readonly]
    pub(super) readable: i32,

    // This field can be neither read nor written by other modules.
    private: i32,
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
