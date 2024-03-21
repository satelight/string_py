fn main() {
    let string_python = string_py::StrLikePy::new("Hello World");
    let data = string_python.slice(2, 23);
    println!("{:?}",data);
}
