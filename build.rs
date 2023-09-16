fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=protocol/api/api.proto");

    tonic_build::configure()
        // .out_dir(out_dir)
        // .compile_well_known_types(true)
        .build_server(false)
        .compile(&["protocol/api/api.proto"], &["protocol/"])
        .unwrap();    
    // prost_build::compile_protos(&["protocol/api/api.proto"], &["protocol/"]).unwrap()
}
