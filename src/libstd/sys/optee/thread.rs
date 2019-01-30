use crate::boxed::FnBox;
use crate::ffi::CStr;
use crate::io;
use crate::sys::{unsupported, Void};
use crate::time::Duration;

pub struct Thread(Void);

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnBox()>)
        -> io::Result<Thread>
    {
        unsupported()
    }

    pub fn yield_now() {
        panic!("unsupported")
    }

    pub fn set_name(_name: &CStr) {
        panic!("unsupported")
    }

    pub fn sleep(_dur: Duration) {
        panic!("unsupported");
    }

    pub fn join(self) {
        match self.0 {}
    }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> { None }
    pub unsafe fn init() -> Option<Guard> { None }
}
