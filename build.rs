use std::env;
use std::path::Path;

fn main() {
    let flycap_root = match env::var("FLYCAP_ROOT") {
        Ok(dir) => dir,
        Err(e) => {
            panic!("Environment variable FLYCAP_ROOT could not be read: {}. (Tip: set to /usr)", e);
        }
    };

    let flycap_path = Path::new(&flycap_root);
    let libdir = flycap_path.join("lib");

    let libname = "flycapture-c";

    println!("cargo:rustc-link-search=native={}",libdir.display());
    // println!("cargo:rustc-link-lib=static={}", libname); // PGR doesn't ship static libs?
    println!("cargo:rustc-link-lib={}", libname);

}
