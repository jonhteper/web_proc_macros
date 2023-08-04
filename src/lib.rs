extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod delete_macro;
mod impl_kind_macro;
mod insert_macro;
mod select_macro;
mod update_macro;

#[proc_macro_derive(ImplKind, attributes(error_kind))]
pub fn impl_kind(input: TokenStream) -> TokenStream {
    impl_kind_macro::impl_kind_macro(input)
}

/// Use insert_stmt_query!(TABLE_NAME, COLUMS_LIST...)
/// # Examples
/// ```
/// use web_proc_macros::insert_stmt_query;
///
/// let query = insert_stmt_query!("table", "col1", "col2");
/// assert_eq!(query, "INSERT INTO table (col1, col2) VALUES (:col1, :col2)");
/// ```
#[proc_macro]
pub fn insert_stmt_query(input: TokenStream) -> TokenStream {
    insert_macro::insert_stmt_macro(input)
}

/// Alias to [update_stmt_query]
#[allow(non_snake_case)]
#[proc_macro]
pub fn INSERT(input: TokenStream) -> TokenStream {
    insert_macro::insert_stmt_macro(input)
}

/// Use select_stmt_query!(TABLE_NAME, COLUMS_LIST..., WHERE_CLAUSE)
/// # Examples
/// ```
/// use web_proc_macros::select_stmt_query;
///
/// let where_clause = "id = :id";
/// let query = select_stmt_query!("table", "col1", "col2", where_clause);
/// assert_eq!(query, "SELECT col1, col2 FROM table WHERE id = :id");    
/// ```
///
/// ```
/// use web_proc_macros::select_stmt_query;
///
/// let query = select_stmt_query!("table", "*", "id = :id");
/// assert_eq!(query, "SELECT * FROM table WHERE id = :id");    
/// ```
/// ```
/// use web_proc_macros::select_stmt_query;
///
/// let query = select_stmt_query!("table", "*");
/// assert_eq!(query, "SELECT * FROM table");    
/// ```
#[proc_macro]
pub fn select_stmt_query(input: TokenStream) -> TokenStream {
    select_macro::select_stmt_macro(input)
}

/// Alias to [select_stmt_query]
#[allow(non_snake_case)]
#[proc_macro]
pub fn SELECT(input: TokenStream) -> TokenStream {
    select_macro::select_stmt_macro(input)
}

/// Use update_stmt_query!(TABLE_NAME, COLUMS_LIST..., WHERE_CLAUSE)
/// # Examples
/// ```
/// use web_proc_macros::update_stmt_query;
///
/// let query = update_stmt_query!("table", "col1", "col2", "id = :id");
/// assert_eq!(query, "UPDATE table SET col1 = :col1, col2 = :col2 WHERE id = :id");    
/// ```
#[proc_macro]
pub fn update_stmt_query(input: TokenStream) -> TokenStream {
    update_macro::update_stmt_macro(input)
}

/// Alias to [update_stmt_query]
#[allow(non_snake_case)]
#[proc_macro]
pub fn UPDATE(input: TokenStream) -> TokenStream {
    update_macro::update_stmt_macro(input)
}

/// Use delete_stmt_query!(TABLE_NAME, WHERE_CLAUSE)
/// # Examples
/// ```
/// use web_proc_macros::delete_stmt_query;
///
/// let query = delete_stmt_query!("table", "id = :id");
/// assert_eq!(query, "DELETE FROM table WHERE id = :id");
/// ```
#[proc_macro]
pub fn delete_stmt_query(input: TokenStream) -> TokenStream {
    delete_macro::delete_stmt_macro(input)
}

/// Alias to [delete_stmt_query]
#[allow(non_snake_case)]
#[proc_macro]
pub fn DELETE(input: TokenStream) -> TokenStream {
    delete_macro::delete_stmt_macro(input)
}

/// Generates a public fields struct.
///
/// WARNING: It's neccessary import macros Serialize and Deserialize to use.
///
/// # Examples
/// ```
/// use web_proc_macros::StructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// #[derive(StructValues)]
/// pub struct User {
///     id: String,
///     name: String,
///     status: u8,
///     groups: Vec<String>,
/// }
///
/// impl User {
///     fn from_values(values: UserValues) -> Self {
///         Self {
///             id: values.id,
///             name: values.name,
///             status: values.status,
///             groups: values.groups
///         }
///     }
/// }
///
/// let _user = User::from_values(
///     UserValues {
///         id: "id.1".to_string(),
///         name: "example".to_string(),
///         status: 0,
///         groups: vec!["Group1".to_string(), "Group2".to_string()],
///     }
/// );
///
///
/// ```
///
/// Is possible to ignore fields with `#[struct_values(skip)]`:
/// ```
/// use web_proc_macros::StructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// #[derive(StructValues)]
/// pub struct User {
///     #[struct_values(skip)]
///     id: String,
///     name: String,
///     status: u8,
///     groups: Vec<String>,
/// }
///
/// let _values = UserValues {
///     name: "example".to_string(),
///     status: 0,
///     groups: vec!["Group1".to_string(), "Group2".to_string()],
/// };
/// ```
#[proc_macro_derive(StructValues, attributes(struct_values))]
pub fn derive_struct_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let values_name = format!("{}Values", name);
    let values_ident = syn::Ident::new(&values_name, name.span());

    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        panic!("StructValues only supports structs with named fields");
    };

    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("StructValues only supports structs with named fields");
    };

    let mut values_fields = Vec::new();
    for field in fields {
        if field
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("struct_values"))
        {
            continue;
        }
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        values_fields.push(quote! {
            pub #field_name: #field_type,
        });
    }

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct #values_ident {
            #(#values_fields)*
        }
    };

    TokenStream::from(expanded)
}

/// Generates a public and full-optional fields struct.
///
/// WARNING: It's neccessary import macros Serialize and Deserialize to use.
///
/// # Examples
/// ```
/// use web_proc_macros::OptStructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// #[derive(OptStructValues)]
/// pub struct User {
///     #[opt_struct_values(skip)]
///     id: String,
///     name: String,
///     status: u8,
///     groups: Vec<String>,
/// }
///
/// let _opt_values = UserOptValues {
///     name: Some("example".to_string()),
///     groups: Some(vec!["Group1".to_string(), "Group2".to_string()]),
///     status: None,
/// };
///
///
/// ```
#[proc_macro_derive(OptStructValues, attributes(opt_struct_values))]
pub fn derive_opt_struct_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let values_name = format!("{}OptValues", name);
    let values_ident = syn::Ident::new(&values_name, name.span());

    let fields = if let Data::Struct(data_struct) = input.data {
        data_struct.fields
    } else {
        panic!("StructValues only supports structs with named fields");
    };

    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("StructValues only supports structs with named fields");
    };

    let mut values_fields = Vec::new();
    for field in fields {
        if field
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("opt_struct_values"))
        {
            continue;
        }
        let field_name = field.ident.unwrap();
        let field_type = field.ty;
        values_fields.push(quote! {
            pub #field_name: Option<#field_type>,
        });
    }

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, Clone, Default)]
        pub struct #values_ident {
            #(#values_fields)*
        }
    };

    TokenStream::from(expanded)
}
