use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_quote, token, Data, DeriveInput, Error, Expr, Field, Fields, Ident, Path, Result, Token,
    Visibility,
};

type Punctuated = syn::punctuated::Punctuated<Field, Token![,]>;

pub fn readonly(input: DeriveInput) -> Result<TokenStream> {
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

    let mut attr_errors = Vec::new();
    let indices = find_and_strip_readonly_attrs(&mut input, &mut attr_errors);

    let original_input = quote! {
        #[cfg(doc)]
        #input
    };

    if !has_defined_repr(&input) {
        input.attrs.push(parse_quote!(#[repr(C)]));
    }

    let mut readonly = input.clone();
    readonly.attrs.insert(0, parse_quote!(#[doc(hidden)]));

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

    input.attrs.insert(0, parse_quote!(#[cfg(not(doc))]));

    let attr_errors = attr_errors.iter().map(Error::to_compile_error);

    Ok(quote! {
        #original_input

        #input

        const _: () = {
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
        };

        #(#attr_errors)*
    })
}

fn has_defined_repr(input: &DeriveInput) -> bool {
    let mut has_defined_repr = false;
    for attr in &input.attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            let path = &meta.path;
            if path.is_ident("C") || path.is_ident("transparent") || path.is_ident("packed") {
                has_defined_repr = true;
            }
            if meta.input.peek(Token![=]) {
                let _value: Expr = meta.value()?.parse()?;
            } else if meta.input.peek(token::Paren) {
                let _group: TokenTree = meta.input.parse()?;
            }
            Ok(())
        });
    }
    has_defined_repr
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

fn find_and_strip_readonly_attrs(input: &mut DeriveInput, errors: &mut Vec<Error>) -> Vec<usize> {
    let mut indices = Vec::new();

    for (i, field) in fields_of_input(input).iter_mut().enumerate() {
        let mut readonly_attr_index = None;

        for (j, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("readonly") {
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                }
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
