use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn insert_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let table_name = match input {
        Expr::Call(call) => call.args[0].clone(),
        _ => panic!("Invalid input"),
    };
    let cols = match input {
        Expr::Call(call) => call.args[1..].to_vec(),
        _ => panic!("Invalid input"),
    };
    let cols_str = cols
        .iter()
        .map(|col| quote! { #col.to_string() + ", " })
        .collect::<proc_macro2::TokenStream>();
    let values_str = cols
        .iter()
        .map(|col| quote! { ":".to_string() + &#col.to_string() + ", " })
        .collect::<proc_macro2::TokenStream>();
    let expanded = quote! {
        {
            let mut cols_str = String::new();
            let mut values_str = String::new();
            cols_str.push_str(&(#cols_str).to_string());
            values_str.push_str(&(#values_str).to_string());
            cols_str.pop();
            cols_str.pop();
            values_str.pop();
            values_str.pop();
            format!("INSERT INTO {} ({}) VALUES ({})", #table_name.to_string(), cols_str, values_str)
        }
    };
    TokenStream::from(expanded)
}
