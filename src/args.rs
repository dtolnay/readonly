// Syntax:
//
//     #[readonly::make(doc = mycratename_doc_cfg)]
//     pub struct MyStruct {...}
//
// If provided, the `doc` attribute will make readonly fields fully pub, meaning
// writeable, if the given cfg is set. This way the readonly fields end up
// visible in rustdoc. The caller is responsible for providing a doc comment
// that explains that the field is readonly.
//
// Intended usage is by placing the following in Cargo.toml:
//
//     [package.metadata.docs.rs]
//     rustdoc-args = ["--cfg", "mycratename_doc_cfg"]

use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

mod kw {
    syn::custom_keyword!(doc);
}

pub struct Args {
    pub doc_cfg: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let doc_cfg = if input.is_empty() {
            None
        } else {
            input.parse::<kw::doc>()?;
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        };

        Ok(Args { doc_cfg })
    }
}
