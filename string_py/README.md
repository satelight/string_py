# string_py

This crate aims to make the String type as easy to use as the str type in python.
This crate is for multibyte String type such as Japanese.

## example

```rust
    let easy_string = EasyString::new("あいうえお");
    let slice_data = easy_string.slice(0, -1);
    assert_eq!("あいうえ",slice_data);
    assert_eq!(5,easy_string.len());
```
