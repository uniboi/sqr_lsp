#![allow(non_snake_case)]

use libc::c_void;
use patches::patch_server;
use retour::static_detour;
use transform::cast;

mod filesystem;
mod libloader;
mod patches;
mod transform;

unsafe fn pseudo_init_csqvm(sv: isize) {
    let DAT_180f2f8b0 = cast!(sv + 0xf2f8b0, *mut u8);
    let FUN_180281870 = cast!(sv + 0x281870, extern "C" fn(*mut u8));

    FUN_180281870(DAT_180f2f8b0);

    let PTR_DAT_180bc8980 = cast!(sv + 0xbc8980, *const u8);
    let sq_createNewVM = cast!(sv + 0x260e0, extern "C" fn(*const u8, i32, f32, i32));

    sq_createNewVM(PTR_DAT_180bc8980, 0, 0.0, 0);
    println!("nothing crashed (woah)");
}

fn main() {
    unsafe {
        let [.., server] = libloader::load_libraries();
        let sv = server.into_raw();
        println!("server.dll base address: {}", sv);

        patch_server(sv);

        pseudo_init_csqvm(sv);
        return; // csqvm init shits itself atm for reasons known only to higher entitier

        let init_csqvm = cast!(sv + 0x289C50, extern "C" fn() -> bool);
        let initialized = init_csqvm();
        println!("csqvm loaded: {initialized}");
    }
}
