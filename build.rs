use std::path::PathBuf;

fn main() -> Result<(), String> {
    let shaders_dir = PathBuf::from("src").join("shaders");
    let resources_dir = PathBuf::from("resources");
    println!("cargo:rerun-if-changed={}", shaders_dir.display());
    println!("cargo:rerun-if-changed={}", resources_dir.display());

    vc4_mesa_compiler::build_shaders_dir(&shaders_dir)?;
    vc4_pack_textures::pack_textures(&resources_dir)?;

    Ok(())
}
