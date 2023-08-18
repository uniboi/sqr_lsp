use std::process::exit;

use libloading::os::windows::Library;

const TIER_0_PATH: &str = "/home/joe/dev/sqr_lsp/target/x86_64-pc-windows-gnu/debug/tier0.dll";
const VSTDLIB_PATH: &str =
    "/home/joe/.steam/steam/steamapps/common/Titanfall2/bin/x64_retail/vstdlib.dll";
const SERVER_PATH: &str = "/home/joe/.steam/steam/steamapps/common/Titanfall2/server.dll";

/// Load a required dll
/// This will exit if there is any error while loading the lib!
unsafe fn load_library(path: &str) -> Library {
    match Library::new(path) {
        Ok(lib) => lib,
        Err(err) => {
            eprintln!("Failed loading Library \"{path}\".\n{err:?}");
            exit(1)
        }
    }
}

/// Loads all required dlls for this binary
/// This will exit if there is any error while loading any library!
pub unsafe fn load_libraries() -> [Library; 3] {
    [
        load_library(TIER_0_PATH),
        load_library(VSTDLIB_PATH),
        load_library(SERVER_PATH),
    ]
}
