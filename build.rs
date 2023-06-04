use std::env;

fn main() {
    // Copy SDL2.dll to target directory
    let out_dir = env::var("OUT_DIR").unwrap();

    let sdl2_file_name = "SDL2.dll";

    let dst = format!("{}/{}", out_dir, sdl2_file_name);

    println!("cargo:rerun-if-changed=src/*");

    match std::fs::copy(sdl2_file_name, dst) {
        Ok(_) => (),
        Err(e) => panic!("Error copying SDL2.dll: {}", e),
    }
}
