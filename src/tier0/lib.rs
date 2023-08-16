mod commandline;
mod mem_alloc;
mod stubs;

#[no_mangle]
extern "system" fn DllMain(_: *const u8, _: u32, _: *const u8) -> u32 {
    1
}
