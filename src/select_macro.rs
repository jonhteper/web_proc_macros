use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitStr, Result,
};

pub struct SelectUpdateQueryInput {
    pub table_name: Expr,
    pub where_clause: Option<Expr>,
    pub cols: Vec<LitStr>,
}

impl Parse for SelectUpdateQueryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let table_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let mut cols = Vec::new();

        let mut where_clause = None;
        while !input.is_empty() {
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            } else if !input.peek2(syn::Token![,]) && !cols.is_empty() {
                where_clause = Some(input.parse()?);
            } else {
                cols.push(input.parse()?);
            }
        }

        Ok(SelectUpdateQueryInput {
            table_name,
            cols,
            where_clause,
        })
    }
}

pub fn select_stmt_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SelectUpdateQueryInput);
    let table_name = &input.table_name;
    let where_clause = &input.where_clause;
    let cols = &input.cols;

    let colums: Vec<_> = cols.iter().map(|col| col.value()).collect();
    let colums = colums.join(", ");

    let expanded = match where_clause {
        Some(clause) => quote! {
            format!(
                "SELECT {} FROM {} WHERE {}",
                #colums,
                #table_name,
                #clause,
            )
        },
        None => quote! {
            format!(
                "SELECT {} FROM {}",
                #colums,
                #table_name,
            )
        },
    };

    TokenStream::from(expanded)
}
