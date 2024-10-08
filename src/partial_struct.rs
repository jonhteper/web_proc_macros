use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, Data, DeriveInput, Fields, Meta, MetaList, NestedMeta, Path};

use quote::quote;

pub fn partial_struct(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("#[derive(PartialStruct)] failed to parse input");
    let (ident, traits) = get_name_traits(&ast);
    let original_ident = ast.ident;
    let lifetimes: Vec<_> = ast.generics.lifetimes().collect();
    let type_params: Vec<_> = ast.generics.type_params().collect();
    let where_clause = &ast.generics.where_clause;

    let fields = if let Data::Struct(data_struct) = ast.data {
        data_struct.fields
    } else {
        panic!("#[derive(PartialStruct)] only supports structs with named fields");
    };

    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("#[derive(PartialStruct)] only supports structs with named fields");
    };

    let mut values_fields = Vec::new();
    let mut fields_in_from = Vec::new();
    for field in fields {
        let mut skip = false;
        let mut pub_field = false;
        let mut other_attrs = Vec::new();
        for attr in &field.attrs {
            if attr.path.is_ident("partial_struct") {
                if let Ok(Meta::List(MetaList { nested, .. })) = attr.parse_meta() {
                    for meta in nested {
                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                            if path.is_ident("skip") {
                                skip = true;
                                break;
                            }

                            if path.is_ident("pub") {
                                pub_field = true;
                            }
                        } else {
                            other_attrs.push(meta.clone());
                        }
                    }
                }
            }
        }
        if skip {
            continue;
        }
        let field_name = field.ident.unwrap();
        let field_type = field.ty;

        if pub_field {
            values_fields.push(quote! {
                #(#[#other_attrs])*
                pub #field_name: #field_type,
            });
        } else {
            let field_vis = field.vis;
            values_fields.push(quote! {
                #(#[#other_attrs])*
                #field_vis #field_name: #field_type,
            });
        }

        fields_in_from.push(quote! {
            #field_name: value.#field_name,
        });
    }

    let expanded = quote! {
        #( #[#traits] )*
        pub struct #ident<#(#lifetimes,)* #(#type_params),*> #where_clause {
            #(#values_fields)*
        }

        impl<#(#lifetimes,)* #(#type_params),*> From<#original_ident> for #ident #where_clause {
            fn from(value: #original_ident) -> #ident {
                #ident {
                    #(#fields_in_from)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_name_traits(input: &DeriveInput) -> (Path, Vec<NestedMeta>) {
    let metas = find_attribute(input, "partial_struct")
        .expect("#[derive(PartialStruct)] requires partial_struct attribute");
    let mut iter = metas.iter();

    if let Some(&NestedMeta::Meta(Meta::Path(ref path))) = iter.next() {
        (path.to_owned(), iter.cloned().collect())
    } else {
        panic!("#[partial_struct(NAME)] attribute requires NAME attribute");
    }
}

/// From https://crates.io/crates/enum-kinds
pub fn find_attribute(
    definition: &DeriveInput,
    name: &str,
) -> Option<Punctuated<NestedMeta, syn::token::Comma>> {
    for attr in definition.attrs.iter() {
        match attr.parse_meta() {
            Ok(Meta::List(MetaList {
                ref path,
                ref nested,
                ..
            })) if path.is_ident(name) => return Some(nested.clone()),
            _ => continue,
        }
    }
    None
}
