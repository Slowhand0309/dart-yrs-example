extern crate dart_sys;

use yrs::{Doc, GetString, Text, Transact};

#[no_mangle]
/// Prints "Hello, world!" to the standard output.
pub extern "C" fn hello_world() {
    let doc = Doc::new();
    let text = doc.get_or_insert_text("article");
    {
        let mut txn = doc.transact_mut();
        text.insert(&mut txn, 0, "hello");
        text.insert(&mut txn, 5, " world");
    }
    println!("{:?}", text.get_string(&doc.transact()));
}