use std::sync::OnceLock;

use libc::c_void;

static COMMAND_LINE: OnceLock<CCommandLine> = OnceLock::new();

#[repr(C)]
struct CCommandLineVTable {
    find_param: fn(this: *mut c_void, param: *mut c_void) -> i32,
    check_param: fn(this: *mut c_void, param: *mut c_void, ret: *mut c_void) -> i32,
}

impl CCommandLineVTable {
    pub fn new() -> Self {
        Self {
            find_param,
            check_param,
        }
    }
}

#[repr(C)]
pub struct CCommandLine<'a> {
    vtable: &'a CCommandLineVTable,
}

#[no_mangle]
extern "C" fn CommandLine() -> &'static CCommandLine<'static> {
    if COMMAND_LINE.get().is_none() {
        let vtable: &'static mut CCommandLineVTable =
            Box::leak(Box::new(CCommandLineVTable::new()));
        COMMAND_LINE.get_or_init(|| CCommandLine { vtable });
    }

    COMMAND_LINE.get().expect("failed retrieving CCommandLine")
}

// always fail, we don't want a working implementation of tier0 anyways
// this could probably just get removed
fn find_param(_a1: *mut c_void, _param: *mut c_void) -> i32 {
    1
}

fn check_param(_a1: *mut c_void, _param: *mut c_void, _ret: *mut c_void) -> i32 {
    1
}
