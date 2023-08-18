#![allow(non_snake_case)]

use retour::static_detour;

mod libloader;

/// cast any arbitrary address to type `t`
macro_rules! cast {
    ($address: expr, $t: ty) => {
        std::mem::transmute::<*const (), $t>($address as _)
    };
}

static_detour! {
    static Test: fn(&'static RecordedAnimation);
}

#[derive(Debug)]
struct RecordedAnimation {
    _gap: [u8; 814],
    refcount: u16,
}

fn add_anim_refcount_hook(anim: &RecordedAnimation) {
    println!("yooo");
}

fn main() {
    // paths are temp obv
    unsafe {
        let [.., server] = libloader::load_libraries();
        let server_handle = server.into_raw();
        println!("server.dll base address: {}", server_handle);

        let create_recorded_animation = cast!(
            server_handle + 0x996e0,
            extern "C" fn() -> &'static RecordedAnimation
        );

        let add_anim_refcount = cast!(server_handle + 0x996c0, fn(&RecordedAnimation));
        let anim = create_recorded_animation();
        // add_anim_refcount(anim);
        // add_anim_refcount(anim);

        Test.initialize(add_anim_refcount, add_anim_refcount_hook).unwrap();
        Test.enable().unwrap();
        add_anim_refcount(&anim);

        println!("refcount is {}", anim.refcount);
    }
}
