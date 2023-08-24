use std::ptr::null;

use libc::c_void;

#[repr(C)]
struct CUtlBuffer {
    data: *mut c_void,
    size: i64, // contains more but in the dll only these are used
}

#[repr(C)]
enum SearchPathAdd {
    PathAddToHead = 0,
    PathAddToTail = 1,
}

#[repr(C)]
struct IFileSystemVTable1 {
    unknown: [*const c_void; 12],
    add_search_path: *const c_void,
    //add_search_path: extern "C" fn(
    //    fs: *mut IFileSystem,
    //    path: *const u8,
    //    path_id: *const u8,
    //    add_type: SearchPathAdd,
    //),
    unknown2: [*const c_void; 82],
    read_from_cache:
        extern "C" fn(fs: *const IFileSystem, path: *const u8, result: *const c_void) -> bool,
    unknown4: [*const c_void; 15],
    mount_vpk: *const c_void,
    // mount_vpk: extern "C" fn(fs: *mut IFileSystem, vpk_path: *const u8), // "was +888" - Blue (god
    // knows what he is
    // talking about)
    unknown5: [*const c_void; 22],
    add_vpk_file: *const c_void,
    //    add_vpk_file: extern "C" fn(
    //        fs: *mut IFileSystem,
    //        basename: *const u8,
    //        add_type: SearchPathAdd,
    //        a1: i64,
    //        a2: i64,
    //        a3: i64,
    //    ),
}

#[repr(C)]
struct IFileSystemVTable2 {
    // I doubt the first parameter is to the vtable itself..
    // ultimately doesn't matter because the parameter is never used (?)
    // null fields are appearantly never used

    // read: extern "C" fn(fs: *const *const Self) -> i32,
    read: *const c_void,
    write: *const c_void,
    open: *const c_void,
    close: *const c_void,
    seek: *const c_void,
    tell: *const c_void,
    size_from_filename: *const c_void,
    size_from_handle: *const c_void,
    flush: *const c_void,
    precache: *const c_void,
    file_exists: *const c_void,
    is_file_writable: *const c_void,
    set_file_writable: *const c_void,
    get_file_time: *const c_void,
    read_file: extern "C" fn(
        fs: *const *const Self,
        filename: *const u8,
        path: *const u8,
        buf: *mut CUtlBuffer,
        max_bytes: i32,
        start_byte: i32,
        fn_alloc: *const c_void, // idk wtf this is supposed to be
    ) -> bool,
    write_file: *const c_void,
    unzip_file: *const c_void,
}

#[repr(C)]
pub struct IFileSystem<'a> {
    vtable1: &'a IFileSystemVTable1,
    vtable2: &'a IFileSystemVTable2,
}

impl IFileSystemVTable1 {
    pub fn new() -> Self {
        Self {
            read_from_cache,
            unknown: [null(); 12],
            add_search_path: null(),
            unknown2: [null(); 82],
            unknown4: [null(); 15],
            mount_vpk: null(),
            unknown5: [null(); 22],
            add_vpk_file: null(),
        }
    }
}

impl IFileSystemVTable2 {
    pub fn new() -> Self {
        Self {
            read_file,
            read: null(),
            write: null(),
            open: null(),
            close: null(),
            seek: null(),
            tell: null(),
            size_from_filename: null(),
            size_from_handle: null(),
            flush: null(),
            precache: null(),
            file_exists: null(),
            is_file_writable: null(),
            set_file_writable: null(),
            get_file_time: null(),
            write_file: null(),
            unzip_file: null(),
        }
    }
}

impl<'a> IFileSystem<'a> {
    pub fn new() -> Self {
        Self {
            vtable1: Box::leak(Box::new(IFileSystemVTable1::new())),
            vtable2: Box::leak(Box::new(IFileSystemVTable2::new())),
        }
    }
}

extern "C" fn read_from_cache(
    _fs: *const IFileSystem,
    _path: *const u8,
    _result: *const c_void,
) -> bool {
    false
}

extern "C" fn read_file(
    _fs: *const *const IFileSystemVTable2,
    filename: *const u8,
    _path: *const u8,
    buf: *mut CUtlBuffer,
    _max_bytes: i32,
    _start_byte: i32,
    _fn_alloc: *const c_void,
) -> bool {
    //TODO
    true
}
