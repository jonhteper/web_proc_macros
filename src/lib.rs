extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result, parse::{Parse, ParseStream}, LitStr};

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


struct InsertQueryInput {
    table_name: LitStr,
    cols: Vec<LitStr>,
}

impl Parse for InsertQueryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let table_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let mut cols = Vec::new();
        while !input.is_empty() {
            cols.push(input.parse()?);
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(InsertQueryInput { table_name, cols })
    }
}

#[proc_macro]
pub fn insert_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InsertQueryInput);
    let table_name = &input.table_name;
    let cols = &input.cols;

    let col_names: Vec<_> = cols.iter().map(|col| format!("{}", col.value())).collect();
    let col_values: Vec<_> = cols.iter().map(|col| format!(":{}", col.value())).collect();

    let col_names = col_names.join(", ");
    let col_values = col_values.join(", ");

    let expanded = quote! {
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            #table_name,
            #col_names,
            #col_values
        )
    };

    TokenStream::from(expanded)
}

// struct InsertQueryInput {
//     table_name: LitStr,
//     cols: Vec<LitStr>,
// }

// impl Parse for InsertQueryInput {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let table_name = input.parse()?;
//         input.parse::<syn::Token![,]>()?;
//         let mut cols = Vec::new();
//         while !input.is_empty() {
//             cols.push(input.parse()?);
//             if input.peek(syn::Token![,]) {
//                 input.parse::<syn::Token![,]>()?;
//             }
//         }
//         Ok(InsertQueryInput { table_name, cols })
//     }
// }

// #[proc_macro]
// pub fn insert_query(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as InsertQueryInput);
//     let table_name = &input.table_name;
//     let cols = &input.cols;

//     let col_names: Vec<_> = cols.iter().map(|col| quote!(#col)).collect();
//     let col_values: Vec<_> = cols.iter().map(|col| quote!(:#col)).collect();

//     let expanded = quote! {
//         format!(
//             "INSERT INTO {} ({}) VALUES ({})",
//             #table_name,
//             #(#col_names),*,
//             #(#col_values),*
//         )
//     };

//     TokenStream::from(expanded)
// }