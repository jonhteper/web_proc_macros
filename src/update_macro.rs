use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::select_macro::SelectUpdateQueryInput;

pub fn update_stmt_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SelectUpdateQueryInput);
    let table_name = &input.table_name;
    let where_clause = &input.where_clause;
    let cols = &input.cols;

    let col_pairs: Vec<_> = cols
        .iter()
        .map(|col| format!("{} = :{}", col.value(), col.value()))
        .collect();
    let col_pairs = col_pairs.join(", ");

    let expanded = quote! {
        format!(
            "UPDATE {} SET {} WHERE {}",
            #table_name,
            #col_pairs,
            #where_clause,
        )
    };

    TokenStream::from(expanded)
}
