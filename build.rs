use rustc_version::{version_meta, Channel};

fn main() {
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=nightly")
    }

    // track_caller is stable since Rust 1.46 (2020), so no version check needed
    println!("cargo:rustc-cfg=track_caller");

    // Add check-cfg for conditional configurations
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
    println!("cargo:rustc-check-cfg=cfg(track_caller)");
    println!("cargo:rustc-check-cfg=cfg(nightly)");
}
