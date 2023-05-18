use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Result,
};

struct DeleteQueryInput {
    table_name: Expr,
    where_clause: Expr,
}

impl Parse for DeleteQueryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let table_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let where_clause = input.parse()?;

        Ok(DeleteQueryInput {
            table_name,
            where_clause,
        })
    }
}

pub fn delete_stmt_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeleteQueryInput);
    let table_name = &input.table_name;
    let where_clause = &input.where_clause;

    let expanded = quote! {
        format!(
            "DELETE FROM {} WHERE {}",
            #table_name,
            #where_clause,
        )
    };

    TokenStream::from(expanded)
}
