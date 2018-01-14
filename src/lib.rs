#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn rust_foo(x: f32) -> i32 {
    unsafe {
        return foo(x) as i32;
    }
}
