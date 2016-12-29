use std::env;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").expect("getting target");

    let libdir = match env::var("FLYCAP_LIBDIR") {
        Ok(dir) => dir,
        Err(_) => {
            match target.as_ref() {
                "x86_64-pc-windows-msvc" | "x86_64-pc-windows-gnu" => {r"C:\Program Files\Point Grey Research\FlyCapture2\lib64\C".into()},
                "i686-pc-windows-msvc" | "i686-pc-windows-gnu" => {r"C:\Program Files\Point Grey Research\FlyCapture2\lib32\C".into()},
                _ => {"/usr/lib".into()},
            }
        }
    };

    let libname = match target.as_ref() {
        "x86_64-pc-windows-msvc" | "i686-pc-windows-msvc" | "x86_64-pc-windows-gnu" | "i686-pc-windows-gnu" => "FlyCapture2_C",
        _ => "flycapture-c",
    };

    let libdir = Path::new(&libdir);

    println!("cargo:rustc-link-search=native={}",libdir.display());
    // println!("cargo:rustc-link-lib=static={}", libname); // PGR doesn't ship static libs?
    println!("cargo:rustc-link-lib={}", libname);

}
