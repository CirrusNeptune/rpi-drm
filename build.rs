use std::path::PathBuf;

fn main() -> Result<(), String> {
    let shaders_dir = PathBuf::from("src").join("shaders");
    println!("cargo:rerun-if-changed={}", shaders_dir.display());

    vc4_mesa_compiler::build_shaders_dir(&shaders_dir)
}
