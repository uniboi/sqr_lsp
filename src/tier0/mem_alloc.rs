use libc::{c_void, malloc};

#[no_mangle]
#[used]
pub static mut g_pMemAllocSingleton: *const c_void = std::ptr::null::<c_void>();

#[no_mangle]
pub extern "C" fn CreateGlobalMemAlloc() -> *mut MemoryAllocator<'static> {
    let vtable: &'static mut MemoryAllocatorVTable =
        Box::leak(Box::new(MemoryAllocatorVTable::new()));
    Box::leak(Box::new(MemoryAllocator { vtable }))
}

#[repr(C)]
struct MemoryAllocatorVTable<'a> {
    unknown1: *mut c_void,
    alloc: fn(this: &'a MemoryAllocator, size: libc::size_t) -> *mut c_void,
    unknown2: *mut c_void,
    realloc: fn(this: &'a MemoryAllocator, mem: *mut c_void, size: usize) -> *mut c_void,
    unknown3: *mut c_void,
    free: fn(this: &'a MemoryAllocator, mem: *mut c_void),
    unknown4: [*mut c_void; 2],
    get_size: fn(this: &'a MemoryAllocator, mem: *mut c_void) -> libc::size_t,
    unknown5: [*mut c_void; 9],
    dump_stats: *mut c_void,
    dump_stats_file_base: *mut c_void,
    unknown6: [*mut c_void; 5],
}

impl<'a> MemoryAllocatorVTable<'a> {
    pub fn new() -> Self {
        Self {
            alloc: _alloc,
            realloc: _realloc,
            free: _free,
            get_size: _get_size,

            // these are appearantly never used in the compiler
            dump_stats: std::ptr::null_mut::<c_void>(),
            dump_stats_file_base: std::ptr::null_mut::<c_void>(),
            unknown1: std::ptr::null_mut::<c_void>(),
            unknown2: std::ptr::null_mut::<c_void>(),
            unknown3: std::ptr::null_mut::<c_void>(),
            unknown4: [std::ptr::null_mut::<c_void>(); 2],
            unknown5: [std::ptr::null_mut::<c_void>(); 9],
            unknown6: [std::ptr::null_mut::<c_void>(); 5],
        }
    }
}

#[repr(C)]
pub struct MemoryAllocator<'a> {
    vtable: &'a MemoryAllocatorVTable<'a>,
}

fn _alloc(_handler: &MemoryAllocator, size: libc::size_t) -> *mut c_void {
    let allocated_mem = unsafe { malloc(size) };
    if allocated_mem as i64 != 0 {
        unsafe { libc::memset(allocated_mem, 0, size) };
    }
    allocated_mem
}

fn _realloc(handler: &MemoryAllocator, mem: *mut c_void, new_size: usize) -> *mut c_void {
    if mem as i64 == 0x0 {
        return _alloc(handler, new_size);
    }

    let old_size = unsafe { libc::_msize(mem) };
    let reallocated_mem: *mut c_void = unsafe { libc::realloc(mem, new_size) };

    if reallocated_mem as i64 != 0 && new_size > old_size {
        unsafe {
            libc::memset(
                (reallocated_mem as i64 + old_size as i64) as *mut c_void,
                0,
                new_size - old_size,
            )
        };
    }

    reallocated_mem
}

fn _free(_handler: &MemoryAllocator, mem: *mut c_void) {
    unsafe { libc::free(mem) }
}

fn _get_size(_handler: &MemoryAllocator, mem: *mut c_void) -> libc::size_t {
    unsafe { libc::_msize(mem) }
}
