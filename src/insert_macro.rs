use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitStr, Result,
};

struct InsertQueryInput {
    table_name: Expr,
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

pub fn insert_stmt_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InsertQueryInput);
    let table_name = &input.table_name;
    let cols = &input.cols;

    let col_names: Vec<_> = cols.iter().map(|col| col.value()).collect();
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
