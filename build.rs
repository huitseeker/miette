fn main() {
    // track_caller is always available with our MSRV (Rust ≥ 1.82)

    // Add check-cfg for conditional configurations
    println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
}
