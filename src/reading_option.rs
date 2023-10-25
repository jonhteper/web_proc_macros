use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn reading_options_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let original_ident = &input.ident;
    let new_ident = Ident::new(&format!("{original_ident}Options"), original_ident.span());

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
        if field
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("reading_options"))
        {
            continue;
        }
        let field_name = field.ident.unwrap();

        new_fields.push(quote! {
            pub #field_name: bool,
        });
    }

    let expanded = quote! {
        #[derive(serde_derive::Serialize, serde_derive::Deserialize, Clone)]
        pub struct #new_ident {
            #(#new_fields)*
        }
    };

    TokenStream::from(expanded)
}
