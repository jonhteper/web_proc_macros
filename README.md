# web_proc_macros
Usefull procedural macros for Web Rust Projects

This crate contains: 

## ImplKind macro
Use example: 
```rust 

use web_proc_macros::ImplKind;
;
#[derive(Debug)]
pub enum MyErrorKind {
    Kind1,
    Kind2,
}

#[derive(Debug)]
#[derive(ImplKind)]
pub enum MyError {
    #[error_kind(MyErrorKind, Kind1)]
    ErrorA,
    #[error_kind(MyErrorKind, Kind2)]
    ErrorB
}

fn main() {
    println!("{:?}", MyError::ErrorA.kind()); // Kind1
}

```

## insert_query macro
Makes an insert query for [mysql](https://crates.io/crates/mysql)

```rust 
#[allow(unused_imports)]

use web_proc_macros::insert_query;


fn main() {
    let query = insert_query!("table_name", "col1", "col2", "col3", "col4");
    println!("{}", query); // INSERT INTO table_name (col1, col2, col3, col4) VALUES (:col1, :col2, :col3, :col4)
}

```
