use crate::args::Args;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_quote, Data, DeriveInput, Error, Field, Fields, Ident, Meta, NestedMeta, Path, Result,
    Token, Visibility,
};

type Punctuated = syn::punctuated::Punctuated<Field, Token![,]>;

pub fn readonly(args: Args, input: DeriveInput) -> Result<TokenStream> {
    let call_site = Span::call_site();

    match &input.data {
        Data::Struct(data) => {
            if data.fields.iter().count() == 0 {
                return Err(Error::new(call_site, "input must be a struct with fields"));
            }
        }
        Data::Enum(_) | Data::Union(_) => {
            return Err(Error::new(call_site, "input must be a struct"));
        }
    }

    let mut input = input;

    let indices = find_and_strip_readonly_attrs(&mut input);

    let original_input = args.doc_cfg.as_ref().map(|doc_cfg| {
        quote! {
            #[cfg(all(#doc_cfg, rustdoc))]
            #input
        }
    });

    if !has_defined_repr(&input) {
        input.attrs.push(parse_quote!(#[repr(C)]));
    }

    let mut readonly = input.clone();
    let input_vis = input.vis.clone();
    let input_fields = fields_of_input(&mut input);
    let readonly_fields = fields_of_input(&mut readonly);

    if indices.is_empty() {
        for field in input_fields {
            field.vis = Visibility::Inherited;
        }
    } else {
        for i in indices {
            input_fields[i].vis = Visibility::Inherited;
            if let Visibility::Inherited = readonly_fields[i].vis {
                readonly_fields[i].vis = input_vis.clone();
            }
        }
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let self_path: Path = parse_quote!(#ident #ty_generics);
    for field in readonly_fields {
        ReplaceSelf::new(&self_path).visit_type_mut(&mut field.ty);
    }

    readonly.ident = Ident::new(&format!("ReadOnly{}", input.ident), call_site);
    let readonly_ident = &readonly.ident;

    if let Some(doc_cfg) = args.doc_cfg {
        let not_doc_cfg = parse_quote!(#[cfg(not(all(#doc_cfg, rustdoc)))]);
        input.attrs.insert(0, not_doc_cfg);
    }

    Ok(quote! {
        #original_input

        #input

        #readonly

        #[doc(hidden)]
        impl #impl_generics core::ops::Deref for #ident #ty_generics #where_clause {
            type Target = #readonly_ident #ty_generics;

            fn deref(&self) -> &Self::Target {
                // Two repr(C) structs with the same fields are guaranteed to
                // have the same layout.
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }
    })
}

fn has_defined_repr(input: &DeriveInput) -> bool {
    for attr in &input.attrs {
        let meta = match attr.parse_meta() {
            Ok(Meta::List(meta)) => meta,
            _ => continue,
        };

        if meta.ident != "repr" || meta.nested.len() != 1 {
            continue;
        }

        let ident = match &meta.nested[0] {
            NestedMeta::Meta(Meta::Word(ident)) => ident,
            _ => continue,
        };

        if ident == "C" || ident == "transparent" || ident == "packed" {
            return true;
        }
    }

    false
}

fn fields_of_input(input: &mut DeriveInput) -> &mut Punctuated {
    match &mut input.data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(fields) => &mut fields.named,
            Fields::Unnamed(fields) => &mut fields.unnamed,
            Fields::Unit => unreachable!(),
        },
        Data::Enum(_) | Data::Union(_) => unreachable!(),
    }
}

fn find_and_strip_readonly_attrs(input: &mut DeriveInput) -> Vec<usize> {
    let mut indices = Vec::new();

    for (i, field) in fields_of_input(input).iter_mut().enumerate() {
        let mut readonly_attr_index = None;

        for (j, attr) in field.attrs.iter().enumerate() {
            if attr.path.is_ident("readonly") {
                readonly_attr_index = Some(j);
                break;
            }
        }

        if let Some(readonly_attr_index) = readonly_attr_index {
            field.attrs.remove(readonly_attr_index);
            indices.push(i);
        }
    }

    indices
}

struct ReplaceSelf<'a> {
    with: &'a Path,
}

impl<'a> ReplaceSelf<'a> {
    fn new(with: &'a Path) -> Self {
        ReplaceSelf { with }
    }
}

impl<'a> VisitMut for ReplaceSelf<'a> {
    fn visit_path_mut(&mut self, path: &mut Path) {
        if path.is_ident("Self") {
            let span = path.segments[0].ident.span();
            *path = self.with.clone();
            path.segments[0].ident.set_span(span);
        } else {
            visit_mut::visit_path_mut(self, path);
        }
    }
}
