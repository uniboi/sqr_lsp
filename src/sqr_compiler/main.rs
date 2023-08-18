#![allow(non_snake_case)]

use libloading::os::windows::Library;

const TIER_0_PATH: &str = "/home/joe/dev/sqr_lsp/target/x86_64-pc-windows-gnu/debug/tier0.dll";
const VSTDLIB_PATH: &str =
    "/home/joe/.steam/steam/steamapps/common/Titanfall2/bin/x64_retail/vstdlib.dll";
const SERVER_PATH: &str = "/home/joe/.steam/steam/steamapps/common/Titanfall2/server.dll";

/// cast any arbitrary address to type `t`
macro_rules! cast {
    ($address: expr, $t: ty) => {
        std::mem::transmute::<*const (), $t>($address as _)
    };
}

#[derive(Debug)]
struct RecordedAnimation {
    _gap: [u8; 814],
    refcount: u16,
}

fn main() {
    // paths are temp obv

    unsafe {
        let m = libc::malloc(128);
        println!("{}", libc::_msize(m));
        let tier0 = Library::new(TIER_0_PATH).unwrap();
        let vstdlib = Library::new(VSTDLIB_PATH).unwrap();
        let server = Library::new(SERVER_PATH).unwrap();

        let server_handle = server.into_raw();
        println!("server.dll base address: {}", server_handle);

        let create_recorded_animation_ptr = (server_handle + 0x996e0) as *const ();
        let create_recorded_animation: extern "C" fn() -> &'static RecordedAnimation =
            std::mem::transmute(create_recorded_animation_ptr);

        let add_anim_refcount = cast!(server_handle + 0x996c0, extern "C" fn(&RecordedAnimation));

        let anim = create_recorded_animation();
        add_anim_refcount(anim);

        println!("anim ptr: {:?}, refcount is {}", anim, anim.refcount);
    }
}
