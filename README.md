# web_proc_macros
Usefull procedural macros for Web Rust Projects

This crate contains: 

## ImplKind macro
Use example: 
```rust 

use web_proc_macros::ImplKind;

#[derive(Debug)]
pub enum MyErrorKind {
    Kind1,
    Kind2,
}

#[derive(Debug)]
#[derive(ImplKind)]
pub enum MyError {
    #[error_kind(MyErrorKind, Kind1)]
    ErrorA(String),
    #[error_kind(MyErrorKind, Kind2)]
    ErrorB,
    #[error_kind(MyErrorKind, Kind2)]
    ErrorC {
        content: String,
        line: usize,
    },
}

fn main() {
    println!("{:?}", MyError::ErrorA("err".to_string()).kind()); // Kind1
}

```

## Macros for [mysql](https://crates.io/crates/mysql) stmt

### INSERT
```rust
use web_proc_macros::insert_stmt_query;

let query = insert_stmt_query!("table", "col1", "col2");
assert_eq!(query, "INSERT INTO table (col1, col2) VALUES (:col1, :col2)");
```

### SELECT
```rust
use web_proc_macros::select_stmt_query;

let where_clause = "id = :id";
let query = select_stmt_query!("table", "col1", "col2", where_clause);
assert_eq!(query, "SELECT col1, col2 FROM table WHERE id = :id");    
```

```rust
use web_proc_macros::select_stmt_query;

let query = select_stmt_query!("table", "*", "id = :id");
assert_eq!(query, "SELECT * FROM table WHERE id = :id");    
```
```rust
use web_proc_macros::select_stmt_query;

let query = select_stmt_query!("table", "*");
assert_eq!(query, "SELECT * FROM table");    
```

### UPDATE
```rust
use web_proc_macros::update_stmt_query;

let query = update_stmt_query!("table", "col1", "col2", "id = :id");
assert_eq!(query, "UPDATE table SET col1 = :col1, col2 = :col2 WHERE id = :id");    
```

### DELETE
```rust
use web_proc_macros::delete_stmt_query;

let query = delete_stmt_query!("table", "id = :id");
assert_eq!(query, "DELETE FROM table WHERE id = :id");
```

## 'Derived' structs macros

## StructValues

Generates a public fields struct.
```rust
use web_proc_macros::StructValues;
use serde_derive::{Serialize, Deserialize};

#[derive(StructValues)]
pub struct User {
    id: String,
    name: String,
    status: u8,
    groups: Vec<String>,
}
impl User {
    fn from_values(values: UserValues) -> Self {
        Self {
            id: values.id,
            name: values.name,
            status: values.status,
            groups: values.groups
        }
    }
}
let _user = User::from_values(
    UserValues {
        id: "id.1".to_string(),
        name: "example".to_string(),
        status: 0,
        groups: vec!["Group1".to_string(), "Group2".to_string()],
    }
);
```

Is possible to ignore fields with `#[struct_values(skip)]`:

```rust
use web_proc_macros::StructValues;
use serde_derive::{Serialize, Deserialize};
#[derive(StructValues)]
pub struct User {
    #[struct_values(skip)]
    id: String,
    name: String,
    status: u8,
    groups: Vec<String>,
}
let _values = UserValues {
    name: "example".to_string(),
    status: 0,
    groups: vec!["Group1".to_string(), "Group2".to_string()],
};
```

## OptStructValues
Generates a public and full-optional fields struct.

 ```rust
 use web_proc_macros::OptStructValues;
 use serde_derive::{Serialize, Deserialize};

 #[derive(OptStructValues)]
 pub struct User {
     #[struct_values(skip)]
     id: String,
     name: String,
     status: u8,
     groups: Vec<String>,
 }

 let _opt_values = UserOptValues {
     name: Some("example".to_string()),
     groups: Some(vec!["Group1".to_string(), "Group2".to_string()]),
     status: None,
 };
 ```

