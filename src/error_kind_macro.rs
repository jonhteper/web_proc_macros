use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, NestedMeta, Path};

use crate::partial_struct::find_attribute;

pub fn error_kind_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let kind_ty = get_kind_ty(&input);

    let name = input.ident;
    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
    } else {
        panic!("ImplKind just can be used in enums");
    };

    let mut kind_variants = Vec::new();

    for variant in variants.clone() {
        let ident = variant.ident;
        if let Some(attr) = variant
            .attrs
            .into_iter()
            .find(|attr| attr.path.is_ident("error_kind"))
        {
            if let Ok(syn::Meta::List(meta)) = attr.parse_meta() {
                if meta.nested.len() == 2 {
                    if let (
                        syn::NestedMeta::Meta(syn::Meta::Path(enum_ty)),
                        syn::NestedMeta::Meta(syn::Meta::Path(variant)),
                    ) = (&meta.nested[0], &meta.nested[1])
                    {
                        kind_variants.push((ident, enum_ty.clone(), Some(variant.clone())));
                    } else {
                        panic!("Invalid value for error_kind");
                    }
                } else if meta.nested.len() == 1 {
                    for sub_meta in meta.nested {
                        if let NestedMeta::Meta(Meta::Path(path)) = sub_meta {
                            if path.is_ident("transparent") {
                                kind_variants.push((ident.clone(), kind_ty.clone(), None));
                            }
                        } else {
                            panic!("Invalid value for #[error_kind]");
                        }
                    }
                } else {
                    panic!("error_kind must have one two arguments");
                }
            } else {
                panic!("Error parsing meta");
            }
        } else {
            panic!("Enum variants must have the attribute `error_kind`");
        }
    }

    let kind_enum = kind_variants
        .first()
        .expect("No variants in Enum")
        .1
        .clone();
    let match_arms = kind_variants.into_iter().map(|(ident, enum_ty, variant)| {
        let fields = &variants.iter().find(|v| v.ident == ident).unwrap().fields;
        match fields {
            syn::Fields::Unit => {
                quote! {
                    Self::#ident => #enum_ty::#variant,
                }
            }
            syn::Fields::Named(_) => {
                quote! {
                    Self::#ident{..} => #enum_ty::#variant,
                }
            }
            syn::Fields::Unnamed(_) => match variant {
                Some(v) => quote! {
                    Self::#ident(..) => #enum_ty::#v,
                },
                None => quote! {
                    Self::#ident(inner) => inner.kind(),
                },
            },
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

fn get_kind_ty(input: &DeriveInput) -> Path {
    let metas = find_attribute(input, "error_kind")
        .expect("#[derive(ErrorKind)] requires error_kind attribute");
    if let Some(&NestedMeta::Meta(Meta::Path(ref path))) = metas.iter().next() {
        path.to_owned()
    } else {
        panic!("#[error_kind(KIND_IDENT)] attribute requires and identifier");
    }
}
