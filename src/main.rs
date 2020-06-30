#![feature(llvm_asm)]

use setjmp_rs::{longjmp, setjmp, JmpBuf};

fn main() {
    let mut env = JmpBuf::new();
    let val = setjmp!(env);

    println!("val is {:?}", val);

    if val.is_none() {
        longjmp!(env, 124);
    }
}
