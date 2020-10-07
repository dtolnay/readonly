Readonly
========

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/readonly-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/readonly)
[<img alt="crates.io" src="https://img.shields.io/crates/v/readonly.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/readonly)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-readonly-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/readonly)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/readonly/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/readonly/actions?query=branch%3Amaster)

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
