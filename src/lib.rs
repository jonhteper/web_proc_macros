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
        panic!("ImplKind solo puede ser utilizado en enums");
    };

    let mut kind_variants = Vec::new();

    for variant in variants {
        let ident = variant.ident;
        if let Some(attr) = variant.attrs.into_iter().find(|attr| attr.path.is_ident("error_kind")) {
            if let Ok(syn::Meta::List(meta)) = attr.parse_meta() {
                if meta.nested.len() == 2 {
                    if let (syn::NestedMeta::Meta(syn::Meta::Path(kind)), syn::NestedMeta::Lit(syn::Lit::Str(variant))) =
                        (&meta.nested[0], &meta.nested[1])
                    {
                        kind_variants.push((ident, kind.clone(), variant.value()));
                    } else {
                        panic!("Valor inválido para error_kind");
                    }
                } else {
                    panic!("error_kind debe tener dos argumentos");
                }
            } else {
                panic!("Error en meta list");
            }
        } else {
            panic!("Variantes deben tener el atributo error_kind");
        }
    }

    let match_arms = kind_variants.into_iter().map(|(ident, kind, variant)| {
        quote! {
            Self::#ident => #kind::#variant,
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn kind(&self) -> ErrorKind {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
