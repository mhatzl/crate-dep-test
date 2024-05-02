#![no_main]
#![no_std]

use mantra_rust_macros::req;

#[req(second_req)]
pub fn pub_fn(n: i32) -> i32 {
    n + 4
}
