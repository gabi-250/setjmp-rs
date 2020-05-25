#![feature(asm)]

use setjmp_rs::{longjmp, setjmp, JmpBuf};

static mut env: JmpBuf = JmpBuf::new();

fn main() {
    unsafe {
        if let Some(v) = setjmp!(env) {
            println!("longjump value={}", v);
        } else {
            println!("{:?}", env);
        }

        println!("calling longjmp!...");
        longjmp!(env, 124);
    }
}
