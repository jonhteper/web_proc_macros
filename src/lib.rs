extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ImplKind, attributes(error_kind))]
pub fn impl_kind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("ImplKind just can be used in enums");
    };

    let mut kind_variants = Vec::new();

    for variant in variants {
        let ident = variant.ident;
        if let Some(attr) = variant.attrs.into_iter().find(|attr| attr.path.is_ident("error_kind")) {
            if let Ok(syn::Meta::List(meta)) = attr.parse_meta() {
                if meta.nested.len() == 2 {
                    if let (syn::NestedMeta::Meta(syn::Meta::Path(kind)), syn::NestedMeta::Meta(syn::Meta::Path(variant))) =
                        (&meta.nested[0], &meta.nested[1])
                    {
                        kind_variants.push((ident, kind.clone(), variant.clone()));
                    } else {
                        panic!("Invalid value for error_kind");
                    }
                } else {
                    panic!("error_kind must have two arguments");
                }
            } else {
                panic!("Error parsing meta");
            }
        } else {
            panic!("Enum variants must have the attribute `error_kind`");
        }
    }

    let kind_enum = kind_variants.first().expect("No variants in Enum").1.clone();
    let match_arms = kind_variants.into_iter().map(|(ident, kind, variant)| {
        quote! {
            Self::#ident => #kind::#variant,
        }
    });
    

    let expanded = quote! {
        impl #name {
            pub fn kind(&self) -> #kind_enum {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
