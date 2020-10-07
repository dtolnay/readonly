//! [![github]](https://github.com/dtolnay/readonly)&ensp;[![crates-io]](https://crates.io/crates/readonly)&ensp;[![docs-rs]](https://docs.rs/readonly)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Struct fields that can be read and written from within the same module, but
//! from outside the module can only be read.
//!
//! # Syntax
//!
//! Place `#[readonly::make]` on a braced struct or tuple struct. This will make
//! all fields of the struct publicly readable according to their individual
//! visibility specifiers, but not writable from other modules.
//!
//! ```
//! mod m {
//!     #[readonly::make]
//!     pub struct S {
//!         pub n: i32,
//!     }
//!
//!     impl S {
//!         pub fn demo(&mut self) {
//!             // Can read and write from inside the same module.
//!             println!("{}", self.n);
//!             self.n += 1;
//!         }
//!     }
//! }
//!
//! fn demo(s: &mut m::S) {
//!     // From outside the module, can only read.
//!     println!("{}", s.n);
//!
//!     // Does not compile:
//!     //s.n += 1;
//! }
//! ```
//!
//! The error appears as follows.
//!
//! ```console
//! error[E0594]: cannot assign to data in a dereference of `m::S`
//!   --> readme.rs:21:5
//!    |
//! 21 |     s.n += 1;
//!    |     ^^^^^^^^ cannot assign
//! ```
//!
//! Optionally, place `#[readonly]` on individual struct fields to make just
//! those fields publicly readable, without affecting other fields of the
//! struct.
//!
//! ```
//! # mod m {
//! #[readonly::make]
//! pub struct S {
//!     // This field can be read (but not written) by super.
//!     #[readonly]
//!     pub(super) readable: i32,
//!
//!     // This field can be neither read nor written by other modules.
//!     private: i32,
//! }
//! # }
//! ```

extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Nothing;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn make(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let original = tokens.clone();
    parse_macro_input!(args as Nothing);
    let input = parse_macro_input!(tokens as DeriveInput);

    expand::readonly(input)
        .unwrap_or_else(|e| {
            let original = proc_macro2::TokenStream::from(original);
            let compile_error = e.to_compile_error();
            quote! {
                #original
                #compile_error
            }
        })
        .into()
}
