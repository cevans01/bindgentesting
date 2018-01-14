extern crate libfoo_sys;
use libfoo_sys::rust_foo;

pub fn main() {
    let x = 3.5;
    let rv = rust_foo(x);
    println!("x = {}", rv);
}
