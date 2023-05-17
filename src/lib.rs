extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Expr, LitStr, Result,
};

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
                        syn::NestedMeta::Meta(syn::Meta::Path(kind)),
                        syn::NestedMeta::Meta(syn::Meta::Path(variant)),
                    ) = (&meta.nested[0], &meta.nested[1])
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

    let kind_enum = kind_variants
        .first()
        .expect("No variants in Enum")
        .1
        .clone();
    let match_arms = kind_variants.into_iter().map(|(ident, kind, variant)| {
        let fields = &variants.iter().find(|v| v.ident == ident).unwrap().fields;
        match fields {
            syn::Fields::Unit => {
                quote! {
                    Self::#ident => #kind::#variant,
                }
            }
            syn::Fields::Named(_) => {
                quote! {
                    Self::#ident{..} => #kind::#variant,
                }
            }
            syn::Fields::Unnamed(_) => {
                quote! {
                    Self::#ident(..) => #kind::#variant,
                }
            }
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

/// Use select_query(TABLE_NAME, COLUMS_LIST...)
/// # Examples
/// ```
/// use web_proc_macros::insert_query;
/// 
/// let query = insert_query!("table", "col1", "col2");
/// assert_eq!(query, "INSERT INTO table (col1, col2) VALUES (:col1, :col2)");
/// ```
#[proc_macro]
pub fn insert_query(input: TokenStream) -> TokenStream {
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

struct SelectUpdateQueryInput {
    table_name: Expr,
    where_clause: Expr,
    cols: Vec<LitStr>,
}

impl Parse for SelectUpdateQueryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let table_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let where_clause = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let mut cols = Vec::new();
        while !input.is_empty() {
            cols.push(input.parse()?);
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(SelectUpdateQueryInput { table_name, where_clause, cols })
    }
}


/// Use select_query(TABLE_NAME, WHERE_CLAUSE, COLUMS_LIST...)
/// # Examples
/// ```
/// use web_proc_macros::select_query;
/// 
/// let query = select_query!("table", "id = :id", "col1", "col2");
/// assert_eq!(query, "SELECT col1, col2 FROM table WHERE id = :id");    
/// ```
/// 
/// ```
/// use web_proc_macros::select_query;
/// 
/// let query = select_query!("table", "id = :id", "*");
/// assert_eq!(query, "SELECT * FROM table WHERE id = :id");    
/// ```
#[proc_macro]
pub fn select_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SelectUpdateQueryInput);
    let table_name = &input.table_name;
    let where_clause = &input.where_clause;
    let cols = &input.cols;

    let colums: Vec<_> = cols.iter().map(|col| col.value()).collect();
    let colums = colums.join(", ");

    let expanded = quote! {
        format!(
            "SELECT {} FROM {} WHERE {}",
            #colums,
            #table_name,
            #where_clause,
        )
    };

    TokenStream::from(expanded)
}



/// Use update_query!(TABLE_NAME, WHERE_CLAUSE, COLUMS_LIST...)
/// # Examples
/// ```
/// use web_proc_macros::update_query;
/// 
/// let query = update_query!("table", "id = :id", "col1", "col2");
/// assert_eq!(query, "UPDATE table SET col1 = :col1, col2 = :col2 WHERE id = :id");    
/// ```
#[proc_macro]
pub fn update_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SelectUpdateQueryInput);
    let table_name = &input.table_name;
    let where_clause = &input.where_clause;
    let cols = &input.cols;

    let col_pairs: Vec<_> = cols.iter().map(|col| format!("{} = :{}", col.value(), col.value())).collect();
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

struct DeleteQueryInput {
    table_name: Expr,
    where_clause: Expr,
}

impl Parse for DeleteQueryInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let table_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let where_clause = input.parse()?;

        Ok(DeleteQueryInput { table_name, where_clause })
    }
}

/// Use delete_query!(TABLE_NAME, WHERE_CLAUSE)
/// # Examples
/// ```
/// use web_proc_macros::delete_query;
/// 
/// let query = delete_query!("table", "id = :id");
/// assert_eq!(query, "DELETE FROM table WHERE id = :id");
/// ```
#[proc_macro]
pub fn delete_query(input: TokenStream) -> TokenStream {
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