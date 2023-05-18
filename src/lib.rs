extern crate proc_macro;

use proc_macro::TokenStream;

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

