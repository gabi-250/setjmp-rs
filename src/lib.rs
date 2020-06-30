/// A struct that stores the information of a calling environment.
#[derive(Debug, Default)]
pub struct JmpBuf<T> {
    /// The instruction pointer.
    pub rip: u64,
    /// The stack pointer.
    pub rsp: u64,
    /// An arbitrary value (specified when calling `longjmp`).
    pub value: Option<T>,
}

impl<T> JmpBuf<T> {
    pub const fn new() -> Self {
        Self {
            rip: 0,
            rsp: 0,
            value: None,
        }
    }
}

/// Stores its calling environment into a `JmpBuf`.
#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! setjmp {
    ($env:expr) => {
        unsafe {
            let mut rip = 0;
            let mut rsp = 0;

            llvm_asm!("lea (%rip), $0" : "=r"(rip));
            llvm_asm!("mov %rsp, $0"   : "=r"(rsp));

            if $env.value.is_none() {
                $env.rip = rip;
                $env.rsp = rsp;
            }

            $env.value
        }

    }
}

/// Performs a long jump based on the specified calling environment.
#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! longjmp {
    ($env:expr, $value:expr) => {
        $env.value = Some($value);
        unsafe {
            llvm_asm!("movq %rax, %rsp
                 jmpq *%rcx"
                 :
                 : "{rcx}"($env.rip), "{rax}"($env.rsp)
                 :
                 );
        }
    }
}
