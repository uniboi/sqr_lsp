extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
use dlopen::{
    raw::{AddressInfoObtainer, Library},
    wrapper::WrapperApi,
};

// TODO add relevant squirrel functions from server.dll
#[derive(WrapperApi)]
struct Api<'a> {
    example_rust_fun: fn(arg: i32) -> u32,
    example_c_fun: unsafe extern "C" fn(),
    example_reference: &'a mut i32,
}

fn main() {
    // paths are temp obv

    unsafe {
        let m = libc::malloc(128);
        println!("{}", libc::_msize(m));
    }

    let tier0_lib =
        Library::open("/home/joe/dev/sqr_lsp/target/x86_64-pc-windows-gnu/debug/tier0.dll")
            .unwrap();
    let vstdlib_lib = Library::open(
        "/home/joe/.steam/steam/steamapps/common/Titanfall2/bin/x64_retail/vstdlib.dll",
    )
    .unwrap();
    return; // just test vstdlib.dll loading for now, shitter refuses to work
    let lib =
        Library::open("/home/joe/.steam/steam/steamapps/common/Titanfall2/server.dll").unwrap();
    let ptr: *const i32 = unsafe { lib.symbol("entry").unwrap() };
    let aio = AddressInfoObtainer::new();
    let addr_info = aio.obtain(ptr as *const ()).unwrap();
    println!("Library base address: {:?}", addr_info.dll_base_addr);
}
