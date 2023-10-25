use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn partial_object_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_ident = &input.ident;
    let new_ident = Ident::new(&format!("Partial{original_ident}"), original_ident.span());
    let lifetimes: Vec<_> = input.generics.lifetimes().collect();
    let type_params: Vec<_> = input.generics.type_params().collect();
    let where_clause = &input.generics.where_clause;

    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        panic!("ReadingOptions only supports structs with named fields");
    };

    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("ReadingOptions only supports structs with named fields");
    };

    let mut new_fields = Vec::new();
    for field in fields {
        let field_name = field.ident.unwrap();
        let field_type = field.ty;

        if field
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("reading_options"))
        {
            new_fields.push(quote! {
                pub #field_name: #field_type,
            });

            continue;
        }

        new_fields.push(quote! {
            pub #field_name: Option<#field_type>,
        });
    }

    let expanded = quote! {
        #[derive(serde_derive::Serialize, serde_derive::Deserialize, Clone)]
        pub struct #new_ident<#(#lifetimes,)* #(#type_params),*> #where_clause {
            #(#new_fields)*
        }
    };

    TokenStream::from(expanded)
}
