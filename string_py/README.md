# string_py

This crate aims to make the String type as easy to use as the str type in python.

## example

```rust
fn string_py_test(){
    let string_data = StrLikePy::new("あいうえお");
    let slice_data = string_data.slice(0, -1);
    println!("{:?}",slice_data);
}
```
