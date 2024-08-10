extern crate proc_macro;

use partial_object::partial_object_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use reading_option::reading_options_macro;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Fields, Token,
};

mod delete_macro;
mod error_kind_macro;
mod impl_kind_macro;
mod insert_macro;
mod partial_object;
mod partial_struct;
mod reading_option;
mod select_macro;
mod update_macro;

/// Create a kind method for struct
/// # Examples
/// ```
/// use web_proc_macros::ErrorKind;
///#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// enum ErrorType {
///     A,
///     B,
///     C,
/// }
///
/// #[derive(ErrorKind)]
/// #[error_kind(ErrorType)]
/// enum CacheError {
///     #[error_kind(ErrorType, A)]
///     Poisoned,
///
///     #[error_kind(ErrorType, B)]
///     Missing,
/// }
///
/// #[derive(ErrorKind)]
/// #[error_kind(ErrorType)]
/// enum ServiceError {
///     #[error_kind(transparent)]
///     Cache(CacheError),
///
///     #[error_kind(ErrorType, C)]
///     Db,
/// }
///
/// assert_eq!(ServiceError::Cache(CacheError::Missing).kind(), ErrorType::B);
/// assert_eq!(ServiceError::Db.kind(), ErrorType::C);
/// ```
#[proc_macro_derive(ErrorKind, attributes(error_kind))]
pub fn error_kind(input: TokenStream) -> TokenStream {
    error_kind_macro::error_kind_macro(input)
}

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

struct StructValuesAttr {
    name: Option<String>,
}

impl StructValuesAttr {
    fn get_identifier(input: &DeriveInput) -> Ident {
        let name = &input.ident;
        let values_name = input
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("struct_values"))
            .and_then(|attr| attr.parse_args::<StructValuesAttr>().ok())
            .and_then(|attr| attr.name)
            .unwrap_or_else(|| format!("{}Values", name));

        Ident::new(&values_name, name.span())
    }

    fn get_identifier_for_opt(input: &DeriveInput) -> Ident {
        let name = &input.ident;
        let values_name = input
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("opt_struct_values"))
            .and_then(|attr| attr.parse_args::<StructValuesAttr>().ok())
            .and_then(|attr| attr.name)
            .unwrap_or_else(|| format!("{}OptValues", name));

        Ident::new(&values_name, name.span())
    }
}

impl Parse for StructValuesAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            if ident == "name" {
                input.parse::<Token![=]>()?;
                name = Some(input.parse::<syn::LitStr>()?.value());
            } else {
                return Err(input.error("expected `name`"));
            }
        }
        Ok(StructValuesAttr { name })
    }
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
///
/// This macro supports lifetimes and generics:
/// ```
/// use web_proc_macros::StructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// pub trait Foo {
///     fn foo(&self) -> bool;
/// }
/// #[derive(Debug, Serialize, Deserialize, Clone)]
/// struct FooImpl;
///
/// impl Foo for FooImpl {
///     fn foo(&self) -> bool {
///         true
///     }
/// }
///
/// #[derive(StructValues)]
/// pub struct User<'a, F>
/// where
///     F: Foo
/// {
///     id: &'a str,
///     name: &'a str,
///     foo: F,
/// }
///
/// let _values = UserValues {
///     id: "example.id",
///     name: "Example User",
///     foo: FooImpl,
/// };
/// ```
///
/// Set the name of the generated structure:
/// ```
/// use web_proc_macros::StructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// #[derive(StructValues)]
/// #[struct_values(name = "UserDestructured")]
/// pub struct User<'a> {
///     id: &'a str,
///     name: &'a str,
/// }
///
/// let _values = UserDestructured {
///     id: "example.id",
///     name: "Example User",
/// };
/// ```
#[proc_macro_derive(StructValues, attributes(struct_values))]
pub fn derive_struct_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let values_ident = StructValuesAttr::get_identifier(&input);
    let lifetimes: Vec<_> = input.generics.lifetimes().collect();
    let type_params: Vec<_> = input.generics.type_params().collect();
    let where_clause = &input.generics.where_clause;

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
        pub struct #values_ident<#(#lifetimes,)* #(#type_params),*> #where_clause {
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
/// ```
///
// This macro supports lifetimes and generics:
/// ```
/// use web_proc_macros::OptStructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// pub trait Foo {
///     fn foo(&self) -> bool;
/// }
/// #[derive(Debug, Serialize, Deserialize, Clone)]
/// struct FooImpl;
///
/// impl Foo for FooImpl {
///     fn foo(&self) -> bool {
///         true
///     }
/// }
///
/// #[derive(OptStructValues)]
/// pub struct User<'a, F>
/// where
///     F: Foo
/// {
///     id: &'a str,
///     name: &'a str,
///     foo: F,
/// }
///
/// let _values = UserOptValues {
///     id: Some("example.id"),
///     name: Some("Example User"),
///     foo: Some(FooImpl),
/// };
///
/// let _values: UserOptValues<'_, FooImpl> = UserOptValues {
///     id: Some("example.id"),
///     name: None,
///     foo: None,
/// };
/// ```
///
/// Set the name of the generated structure:
/// ```
/// use web_proc_macros::OptStructValues;
/// use serde_derive::{Serialize, Deserialize};
///
/// #[derive(OptStructValues)]
/// #[opt_struct_values(name = "UserInput")]
/// pub struct User<'a> {
///     id: &'a str,
///     name: &'a str,
/// }
///
/// let _values = UserInput {
///     id: Some("example.id"),
///     ..Default::default()
/// };
/// ```
#[proc_macro_derive(OptStructValues, attributes(opt_struct_values))]
pub fn derive_opt_struct_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let values_ident = StructValuesAttr::get_identifier_for_opt(&input);
    let lifetimes: Vec<_> = input.generics.lifetimes().collect();
    let type_params: Vec<_> = input.generics.type_params().collect();
    let where_clause = &input.generics.where_clause;

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
        pub struct #values_ident<#(#lifetimes,)* #(#type_params),*> #where_clause {
            #(#values_fields)*
        }
    };

    TokenStream::from(expanded)
}

/// Create a new struct to select what fields the client need.
/// The name of the new struct uses the notation `<NAME>Options`
/// by default.
///
/// # Examples
/// ```rust
/// use web_proc_macros::ReadingOptions;
/// use serde_derive;
///
/// #[derive(ReadingOptions)]
/// pub struct User {
///     id: String,
///     email: String,
///     alias: String,
///     is_active: bool,
/// }
///
/// let user_options = UserOptions {
///     id: true,
///     email: false,
///     alias: true,
///     is_active: false,
/// };
/// ```
/// Because this macro uses "white list" pattern, you can specify
/// "always incude" fields:
/// ```rust
/// use web_proc_macros::ReadingOptions;
/// use serde_derive;
///
/// #[derive(ReadingOptions)]
/// pub struct User {
///     #[reading_options(always)]
///     id: String,
///     email: String,
///     alias: String,
///     is_active: bool,
/// }
///
/// let user_options = UserOptions {
///     email: false,
///     alias: true,
///     is_active: false,
/// };
/// ```
#[proc_macro_derive(ReadingOptions, attributes(reading_options))]
pub fn derive_reading_options(input: TokenStream) -> TokenStream {
    reading_options_macro(input)
}

/// WARNING: this macro only can work with `ReadingOptions` macro.
///
/// Generate a new struct with optional fields, except if there uses
/// `#[reading_options(always)]`.
///
/// The name of the new struct uses the notation `Partial<NAME>`
/// by default.
///
/// # Examples
/// ```rust
/// use web_proc_macros::{ReadingOptions, PartialObject};
/// use serde_derive;
///
/// #[derive(ReadingOptions, PartialObject)]
/// pub struct User {
///     #[reading_options(always)]
///     id: String,
///     email: String,
///     alias: String,
///     is_active: bool,
/// }
///
/// let user_options = PartialUser {
///     id: "John Doe".to_string(),
///     email: None,
///     alias: None,
///     is_active: None,
/// };
/// ```
#[proc_macro_derive(PartialObject)]
pub fn derive_partial_object(input: TokenStream) -> TokenStream {
    partial_object_macro(input)
}

/// Create a 'sub-struct'.
#[proc_macro_derive(PartialStruct, attributes(partial_struct))]
pub fn derive_partial_struct(input: TokenStream) -> TokenStream {
    partial_struct::partial_struct(input)
}
