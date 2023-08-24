use crate::{filesystem::IFileSystem, transform::cast};
use libc::c_void;

#[repr(C)]
struct VEngineServerVTable {
    unknown_0: *mut c_void,
    unknown_4: extern "C" fn(_: i64) -> bool,
    unknown_5: *mut c_void,
    unknown_18: extern "C" fn(_: *mut c_void, _: i16) -> i32,
}

#[repr(C)]
struct VEngineServer<'a> {
    vtable: &'a VEngineServerVTable,
}

#[repr(C)]
struct CGlobals {
    gap_0: [u8; 16],
    time: f32,
    gap_1: [u8; 36],
    isMp: i32,
    map_name: &'static std::ffi::CStr,
    gap_2: [u8; 16],
    unknown_78: i32,
    gap7c: [u8; 4],
    unknown_ptr: *mut c_void,
    gap_end: [u8; 1000],
}

extern "C" fn engine_server_unknown4_stub(_: i64) -> bool {
    true
}

extern "C" fn engine_server_unknown18_stub(_: *mut c_void, _: i16) -> i32 {
    1
}

pub fn patch_server(sv: isize) {
    let globals = CGlobals {
        time: 0.0,
        map_name: Box::leak(Box::new(std::ffi::CString::new("mp_rise").unwrap())),
        isMp: 1,
        unknown_ptr: unsafe { libc::malloc(0x4000) },
        gap_0: [0; 16],
        gap_1: [0; 36],
        gap_2: [0; 16],
        gap7c: [0; 4],
        gap_end: [0; 1000],
        unknown_78: 0,
    };

    unsafe {
        let mut _g_globals = cast!(sv + 0xBFBE08, *mut CGlobals);
        _g_globals = Box::leak(Box::new(globals));

        let mut _g_engine_sv = cast!(sv + 0xBFBD98, *mut VEngineServer);
        _g_engine_sv = Box::leak(Box::new(VEngineServer {
            vtable: &VEngineServerVTable {
                unknown_4: engine_server_unknown4_stub,
                unknown_18: engine_server_unknown18_stub,
                unknown_0: std::ptr::null_mut(),
                unknown_5: std::ptr::null_mut(),
            },
        }));

        let mut _g_fileInterface = cast!(sv + 0x1616730, *mut IFileSystem);
        _g_fileInterface = Box::leak(Box::new(IFileSystem::new()));
    };

    println!("PATCHED SERVER");
}
