#![feature(asm)]

#[derive(Debug, Default)]
pub struct JmpBuf {
    pub rip: u64,
    pub rsp: u64,
    pub value: Option<u64>,
}

impl JmpBuf {
    pub const fn new() -> JmpBuf {
        JmpBuf {
            rip: 0,
            rsp: 0,
            value: None,
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! setjmp {
    ($env:expr) => {
        unsafe {
            asm!("lea (%rip), $0" : "=r"($env.rip));
            asm!("mov %rsp, $0"   : "=r"($env.rsp));

            $env.value
        }

    }
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! longjmp {
    ($env:expr, $value:expr) => {
        println!("longjmp! macro");
        $env.value = Some($value);
        unsafe {
            asm!("movq %rax, %rsp
                 jmpq *%rcx"
                 :
                 : "{rcx}"($env.rip), "{rax}"($env.rsp)
                 :
                 );
        }
    }
}
