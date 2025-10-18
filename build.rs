use rustc_version::{version_meta, Channel};

fn main() {
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=nightly")
    }

    // Configure track_caller based on Rust version
    if let Ok(version) = rustc_version::version() {
        if version >= rustc_version::Version::new(1, 46, 0) {
            println!("cargo:rustc-cfg=track_caller");
        }
    }

    // Add check-cfg for conditional configurations
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
    println!("cargo:rustc-check-cfg=cfg(track_caller)");
    println!("cargo:rustc-check-cfg=cfg(nightly)");
}
